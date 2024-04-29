import { JSDOM, ResourceLoader } from "jsdom";
import { MongoClient } from "mongodb";

interface WheelingChair {
  name: string;
  details: Map<string, string>;
  image: string;
  price: string;
}


async function save(wheelingChairs: any[]): Promise<void> {
  const client = new MongoClient("mongodb://admin:admin@127.0.0.1:30000");

  try {
    // Connect to the MongoDB client
    await client.connect();

    // Get a reference to the database and collection
    const db = client.db("Rustaurant");
    const collection = db.collection("wheelingChairs");

    // Insert the wheelchairs into the collection
    const result = await collection.insertMany(wheelingChairs);

    console.log(`Inserted ${result.insertedCount} wheeling chairs into the database.`);
  } catch (error) {
    console.error('Error while inserting into MongoDB:', error);
  } finally {
    // Ensure the client is closed to avoid memory leaks
    await client.close();
  }
}

const wheelingChairsUrl = "https://monfauteuilroulant.com/Fauteuils-Roulants/Fauteuil-roulant-standard";

async function getWheelingChairsList(): Promise<string[]> {
  const resourceLoader = new ResourceLoader({
    strictSSL: false
  })

  const dom = await JSDOM.fromURL(wheelingChairsUrl, {
    resources: resourceLoader
  });

  const { document } = dom.window;

  let wheelingChairsList: NodeListOf<Element>;

  let result: string[] = [];

  try {
    wheelingChairsList = document.querySelectorAll(".product-name h3 a")
  } catch (e) {
    console.log(e);
    return [];
  }
  let href: string | null;
  for (const wheelingChair of wheelingChairsList) {
    href = wheelingChair.getAttribute("href");
    if (href) {
      result.push(href);
    }
  }

  return result;
}

async function getWheelingChairDetails(url: string): Promise<WheelingChair> {
  const resourceLoader = new ResourceLoader({
    strictSSL: false
  })

  let dom: JSDOM;

  try {
    dom = await JSDOM.fromURL(url, {
      resources: resourceLoader
    });

  } catch (e) {
    console.log(e);
    return {
      name: "",
      details: new Map<string, string>(),
      image: "",
      price: ""
    }
  }

  const { document } = dom.window;

  const name = document.querySelector("h1")?.textContent;

  const details = new Map<string, string>();

  const detailsListContent = document.querySelectorAll(".product-desc .row .table tbody tr");

  for (const detail of detailsListContent) {
    const detailRow = detail.querySelectorAll("td");
    details.set(detailRow[0].textContent || "", detailRow[1].textContent || "");
  }

  const image = document.querySelector(".hide-bullets .thumbnail img")?.getAttribute("src");

  return {
    name: name || "",
    details: details,
    image: image || "",
    price: document.querySelector(".price")?.textContent || ""
  }
}

function wheelingChairToMongoDocument(wheelingChair: WheelingChair): any {
  const doc: any = {
    name: wheelingChair.name,
    image: wheelingChair.image,
    price: wheelingChair.price
  }

  for (const [key, value] of wheelingChair.details.entries()) {
    let newKey = key.toLowerCase().replace(/ /g, "_").replace(/è/, "e").replace(/é/g, "e").replace(/à/g, "a").replace(/ç/g, "c").replace(/-/g, "_");

    if (value.includes("cm")) {
      doc[newKey] = parseInt(value.split(" ")[0].split("-")[0]) / 100;
      if (isNaN(doc[newKey])) {
        if (value.includes("De ")) {
          doc[newKey] = parseInt(value.split(" ")[1]) / 100;
        } else {
          doc[newKey] = value;
        }
      }
    } else {

      doc[newKey] = value;
    }
  }
  return doc;
}


async function main() {
  const wheelingChairsList = await getWheelingChairsList();

  const wheelingChairsDetails = await Promise.all(wheelingChairsList.map(getWheelingChairDetails));

  const mongoDocuments = wheelingChairsDetails.map(wheelingChairToMongoDocument);

  await save(mongoDocuments);
}

main();