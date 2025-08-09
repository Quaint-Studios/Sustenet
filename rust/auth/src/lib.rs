// This is the auth server. It handles all authentication from everywhere.
// I may also eventually make this distributed as well depending on the load.
// The idea is that the auth server will be a trusted endpoint for all clusters and clients.

// If some random person wants to host a server, how do we handle authentication without trusting them with the password? Simple, we're the middleman. For every server.

// 1. The client tells the cluster, "Yo, i'd like to authenticate."
// 2. The untrusted cluster tells their client, "Yo, your secret id is `5d600d55-2261-4b12-a543-3dc2f6f54a81`."
// 3. The Trusted Auth Server gets a message from the untrusted cluster and says, "Alright, I'll save your UUID for 30 seconds."
// 4. The client sends their UUID with their credentials.
// 5. The auth server says to the cluster server, "Yeah, they're good. Bye."
// 6. The untrusted cluster tells the client, "Alright, bossman said you're good to go. Come in."
// 7. The cluster server also gets a UID they can track every few seconds to if they wanted to. This is good for security like changing passwords and triggering an optional "sign out all clients". This will be done with a WebSocket server that publishes when a specific UID wants to be logged out. This is safe because usernames and emails are never public. Only UID and Display names. So even if someone knew a specific UID was changed recently, they have no way to really target that user. Especially because we'll be enforcing that you can never have your username the same as your display name UNLESS you have 2FA. If you have 2FA then we care a little less. Still not safe.

// Why do clusters from untrusted servers need authentication?
// We want to still be able to get their purchases.
// The cluster will get all of the purchases for the UID from the auth server and then send them to the client.
// The cluster will have a copy of the purchases.

// An idea to improve security so people don't just spam for every possible UID is to have a token the auth server encrypts
// and gives it to the cluster server. The cluster server has to send that token back with the UID to get the purchases.
// It'll also send an expiration time for the token. The token is just the UID and the expiration time.

// Additionally, on a successful auth, the auth will send the client a token that allows them to access all of their data.
// THIS token should never be shared with the server. It's an actual token to their account. The cluster server can tell the
// client what their UID is at this point to save bandwidth on the auth server.