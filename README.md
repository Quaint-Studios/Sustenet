# Sustenet
A C# networking solution for Unity3D that has a primary focus on scaling by allowing multiple servers to work together.

## Vision

*This is a rough vision of where this project is headed, a more detailed layout will eventually be added.*

The goal for Sustenet is to develop a connetion of servers. There are four major components in Sustenet.

- The `Master` server is a central hub where all clusters go to be registered. There should really only be one Master Server. But I can't stop you if you want to do something more with it.
- The `Cluster` would be considered your traditional server. You load Sustenet as a Cluster and it contains some configurations:
    - `Key` - The Cluster has a key in it, much like the SSH key you place on a server. Each Cluster should have a unique key. But, like I said, I can't stop you. You can reuse keys if you'd like. Just be aware that if one key is compromised, they all are. I will need some more research on how much security is required in an instance like this. Or what other approaches are an option.
    - `Master Server IP` - This is just telling the Cluster what IP to register to.
    - `[Master Server Port = 6256]` - Again, just some information on the Master Server.
    - `[Connection Limit = 0]` - This is an optional property. Since it's set to 0, no connection limit is being enforced.
    - *more soon...*
- The `Fragment` is only used if you want to break down Clusters. An example of this would be where a certain section of a map belongs to a fragment. When the player moves enough out of the Fragment's zone, they'll be transferred to another fragment. This should work well if the Fragment servers are all on a local network. Primarily because if the user is on the border of the Fragment's zone, then it would actually overlap with the border of another Fragment's zone. This overlap means that the user should actually get an updates from both servers. Not duplicates, of course. But if there was another player in the edge of that zone but on a different Fragment server, we'd still want to see them.
- The `Client` is simply a Client. They'll connect to the Master server and have two options:
    - Login immediately, joining whatever Cluster they've been automatically allocated to, based on how much traffic the other Clusters are experiencing.
    - Manaully select a cluster, browsing their names and other information. If there's a connection limit, then lock access to join that server.

    That's it. After that, they'll just send and receive messages from their Cluster.

Sustenet is aiming to improve this methodology over time. It's a learning experience. The structure may change over time. This will be the route taken for now though.

# Collaboration
While I am still in the process of designing the structure of this project, I will not be actively accepting any callaborative efforts via pull requests. I am, however, open to being pointed in certain directions. Articles and documentation on specific issues are greatly appreciated. Even discussing it directly is welcome. If you're interested in that, feel free to join my [Discord](https://discord.makosai.com). You can discuss more about it in the #sustenet channel.