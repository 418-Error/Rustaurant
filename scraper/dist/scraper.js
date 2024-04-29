"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const jsdom_1 = require("jsdom");
const mongodb_1 = require("mongodb");
function save(wheelingChairs) {
    return __awaiter(this, void 0, void 0, function* () {
        const client = new mongodb_1.MongoClient("mongodb://admin:admin@127.0.0.1:30000");
        try {
            // Connect to the MongoDB client
            yield client.connect();
            // Get a reference to the database and collection
            const db = client.db("Rustaurant");
            const collection = db.collection("wheelingChairs");
            // Insert the wheelchairs into the collection
            const result = yield collection.insertMany(wheelingChairs);
            console.log(`Inserted ${result.insertedCount} wheeling chairs into the database.`);
        }
        catch (error) {
            console.error('Error while inserting into MongoDB:', error);
        }
        finally {
            // Ensure the client is closed to avoid memory leaks
            yield client.close();
        }
    });
}
const wheelingChairsUrl = "https://monfauteuilroulant.com/Fauteuils-Roulants/Fauteuil-roulant-standard";
function getWheelingChairsList() {
    return __awaiter(this, void 0, void 0, function* () {
        const resourceLoader = new jsdom_1.ResourceLoader({
            strictSSL: false
        });
        const dom = yield jsdom_1.JSDOM.fromURL(wheelingChairsUrl, {
            resources: resourceLoader
        });
        const { document } = dom.window;
        let wheelingChairsList;
        let result = [];
        try {
            wheelingChairsList = document.querySelectorAll(".product-name h3 a");
        }
        catch (e) {
            console.log(e);
            return [];
        }
        let href;
        for (const wheelingChair of wheelingChairsList) {
            href = wheelingChair.getAttribute("href");
            if (href) {
                result.push(href);
            }
        }
        return result;
    });
}
function getWheelingChairDetails(url) {
    return __awaiter(this, void 0, void 0, function* () {
        var _a, _b, _c;
        const resourceLoader = new jsdom_1.ResourceLoader({
            strictSSL: false
        });
        let dom;
        try {
            dom = yield jsdom_1.JSDOM.fromURL(url, {
                resources: resourceLoader
            });
        }
        catch (e) {
            console.log(e);
            return {
                name: "",
                details: new Map(),
                image: "",
                price: ""
            };
        }
        const { document } = dom.window;
        const name = (_a = document.querySelector("h1")) === null || _a === void 0 ? void 0 : _a.textContent;
        const details = new Map();
        const detailsListContent = document.querySelectorAll(".product-desc .row .table tbody tr");
        for (const detail of detailsListContent) {
            const detailRow = detail.querySelectorAll("td");
            details.set(detailRow[0].textContent || "", detailRow[1].textContent || "");
        }
        const image = (_b = document.querySelector(".hide-bullets .thumbnail img")) === null || _b === void 0 ? void 0 : _b.getAttribute("src");
        return {
            name: name || "",
            details: details,
            image: image || "",
            price: ((_c = document.querySelector(".price")) === null || _c === void 0 ? void 0 : _c.textContent) || ""
        };
    });
}
function wheelingChairToMongoDocument(wheelingChair) {
    const doc = {
        name: wheelingChair.name,
        image: wheelingChair.image,
        price: wheelingChair.price
    };
    for (const [key, value] of wheelingChair.details.entries()) {
        let newKey = key.toLowerCase().replace(/ /g, "_").replace(/è/, "e").replace(/é/g, "e").replace(/à/g, "a").replace(/ç/g, "c").replace(/-/g, "_");
        if (value.includes("cm")) {
            doc[newKey] = parseInt(value.split(" ")[0].split("-")[0]) / 100;
            if (isNaN(doc[newKey])) {
                if (value.includes("De ")) {
                    doc[newKey] = parseInt(value.split(" ")[1]) / 100;
                }
                else {
                    doc[newKey] = value;
                }
            }
        }
        else {
            doc[newKey] = value;
        }
    }
    return doc;
}
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const wheelingChairsList = yield getWheelingChairsList();
        const wheelingChairsDetails = yield Promise.all(wheelingChairsList.map(getWheelingChairDetails));
        const mongoDocuments = wheelingChairsDetails.map(wheelingChairToMongoDocument);
        yield save(mongoDocuments);
    });
}
main();
//# sourceMappingURL=scraper.js.map