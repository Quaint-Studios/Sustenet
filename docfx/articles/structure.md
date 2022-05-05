# Structure
This article will explain the overall structure of the entire project and how things function.

## Master Server
The master server is the server that is responsible for keeping track of all the cluster servers and clients.

It does the following:

1. Loads all RSA public keys.
2. Loads all AES keys.
3. Initializes data.
	4. Sets up the packet handler with commands such as:
		* `answerPassphrase` - Checks to see if the response is equal to the cluster client's name. The cluster client's name is actually the passphrase and that's how it's compared.
		* `validateCluster`- Reads the packet for a key name provided by a cluster server. Then it generates a passphrase and encrypts it using that key. The cluster is required to decrypt the passphrase and send it back to the master server.
		* `validateLogin` - Reads the packet for a username that is 3 characters or longer. Then it sends a packet with the code `1` to the client. This code signals the `initializeLogin` packet on the client.
			* In the future this will actually be an auth layer that will validate a token or password securely. Right now it just takes a username and validates the length.
        
Scenario:

1. The master server starts.
2. A cluster server connects, validates itself, and gets registered.
3. A client connects to the master server.
4. The client is assigned an id in incremental order.
5. The client logs in.
6. The client is assigned their requested username.
7. The client requests to be sent to a cluster server OR requests a list of cluster servers and then requests to join a cluster server specifically.
8. The client sends a request to move their vector to a certain position.

## Cluster Server

## Client