// use async_graphql::{Object, ID};

// use super::model::{Contact, Restaurant};

// #[Object]
// impl Restaurant{
//     async fn id(&self) -> ID {
//         self.id.to_string().into()
//     }

//     async fn name(&self) -> &String {
//         &self.name
//     }

//     async fn addr(&self) -> &String {
//         &self.addr
//     }

//     async fn contact(&self) -> &Contact {
//         &self.contact
//     }
// }

// #[Object]
// impl Contact {
//     async fn phone(&self) -> &String {
//         &self.phone
//     }

//     async fn email(&self) -> &String {
//         &self.email
//     }
// }