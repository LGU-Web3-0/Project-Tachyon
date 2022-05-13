//! Session middleware is responsible for pass user information to the server
//!Cookies are small text files that a website stores on your device (smartphones, computers etc.) when you browse the internet. They are created when your browser loads a particular website, and the site sends information to your browser which then creates a text file. Cookies can store a range of information, including personal data (such as name, home address, email address) and information about your preferred language or location etc. that allows the site to present you with information customized to fit your needs.
//! * What are session cookies?
//!
//! Session cookies are cookies that last for a session. A session starts when you launch a website or web app and ends when you leave the website or close your browser window. Session cookies contain information that is stored in a temporary memory location which is deleted after the session ends. Unlike other cookies, session cookies are never stored on your device. Therefore, they are also known as transient cookies, non-persistent cookies, or temporary cookies.
//! * How do session cookies work?
//!
//!The session cookie is a server-specific cookie that cannot be passed to any machine other than the one that generated the cookie. The server creates a “session ID” which is a randomly generated number that temporarily stores the session cookie. This cookie stores information such as the user’s input and tracks the movements of the user within the website. There is no other information stored in the session cookie.
//!session cookies working
//!Session cookies are set on a device’s temporary memory when a browser session starts.
//!What is the purpose of session cookies?
//!
//!A website itself cannot track a user’s movement on its webpage and treats each new page request as a new request from a new user. Session cookies allow websites to remember users within a website when they move between web pages. These cookies tell the server what pages to show the user so the user doesn’t have to remember where they left off or start navigating the site all over again. Therefore, without session cookies, websites have no memory. Session cookies are vital for user experience on online shops and websites when the functionalities depend on users’ activities.
//!What are session cookies examples?
//!
//!The most common example of a session cookie in action is the shopping cart on eCommerce websites. When you visit an online shop and add items to your shopping cart, the session cookie remembers your selection so your shopping cart will have the items you selected when you are ready to checkout. Without session cookies, the checkout page will not remember your selection and your shopping cart will be empty.
//!
//!Session cookies also help users to browse and add items to the shopping cart without logging in on an eCommerce site. Only when users checkout, do they have to add their name, address, and payment information.
//!

use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// This is the struct of management struct, which basically cover the Permissions management
/// functionality. For the admin, he/she can have user management priviledge. So the corresponding
/// bool is true.
#[derive(Serialize, Deserialize, Debug)]
pub struct Permissions {
    pub task_management: bool,
    pub file_management: bool,
    pub team_management: bool,
    pub user_management: bool,
    pub system_management: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub perms: Permissions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordForgot {
    pub email: String,
    pub token: Uuid,
}
