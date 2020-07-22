# Sustenet
A C# networking solution for Unity3D that has a primary focus on scaling by allowing multiple servers to work together.

![.NET Core](https://github.com/Quaint-Studios/Sustenet/workflows/.NET%20Core/badge.svg) [![CodeFactor](https://www.codefactor.io/repository/github/quaint-studios/sustenet/badge)](https://www.codefactor.io/repository/github/quaint-studios/sustenet)

## Vision

*This is a rough vision of where this project is headed, a more detailed layout will eventually be added.*

The goal for Sustenet is to develop a connetion of servers. There are four major components in Sustenet.

- The `Master` server is where all clusters go to be registered. There should really only be one Master Server. But I can't stop you if you want to do something more with it.
- The `Cluster` would be considered your traditional server. You load Sustenet as a Cluster and it contains some configurations:
    - `Key` - The Cluster has a key in it, much like the SSH key you place on a server. Each Cluster should have a unique key. But, like I said, I can't stop you. You can reuse keys if you'd like. Just be aware that if one key is compromised, they all are. I will need some more research on how much security is required in an instance like this. Or what other approaches are an option.
    - `Master Server IP` - This is just telling the Cluster what IP to register to.
    - `[Master Server Port = 6256]` - Again, just some information to properly connect to the Master Server.
    - `[Connection Limit = 0]` - This is an optional property. Since it's set to 0, no connection limit is being enforced.
    - *more soon...*
- The `Fragment` is used to give different settings to certain areas in a Cluster. This includes the size of the Fragment in-game, the amount of players in it before the settings might change, keeping track of which players are in this Fragment, and update-rates.
- The `Client` is simply a Client. They'll connect to the Master server and have two options:
    - Login immediately, joining whatever Cluster they've been automatically allocated to, based on how much traffic the other Clusters are experiencing or based on their ping.
    - Manaully select a cluster, browsing their names and other information. If there's a connection limit, then lock access to join that server.

    That's it. After that, they'll just send and receive messages from their Cluster and the Cluster will handle swapping the player between Fragments based on their position.

Sustenet is aiming to improve this methodology over time. It's a learning experience. The structure may change over time. This will be the route taken for now though.

# Project Roadmaps

### Legends
- :heavy_check_mark: Task has been completed.
- :x: Task has not been completed.

- **planned:** A proposal has been made to implement this but isn't guaranteed.
- **not started:** It hasn't been started but is being prepared.
- **in progress:** Currently in development.
- **in review:** The task has been developed and is functional. It still needs to undergo testing for bugs.
- **completed:** This task has been reviewed and has been deemed functional for production.

### Core Roadmap
These are tasks for the core portion of SustenetUnity. This includes everything that would be exported into a DLL as well. It's anything that makes the project function. Or, more specifically, just about anything under the `src` folder but not exclusively (i.e. the cfg folder could be included).

#### Base Connection
| Task              | Current Status    | Finished      | 
|-------------------|-------------------|---------------|
| Handle TCP/UDP disconnections, both intended and accidental. | in review | :heavy_check_mark:
| Have the client properly disconnect the TCP/UDP connection from the server intentionally. | in progress | :x:

#### TCP
| Task              | Current Status    | Finished      | 
|-------------------|-------------------|---------------|
| Establish a TCP connection with a client. | in review | :heavy_check_mark:
| Handle disconnections, both intended and accidental. | in review | :heavy_check_mark:
| Have the client properly disconnect the TCP connection from the server intentionally. | in progress | :x:

#### UDP
| Task              | Current Status    | Finished      | 
|-------------------|-------------------|---------------|
| Establish a UDP connection with a client. | in review | :heavy_check_mark:

#### Data Transfer
| Task              | Current Status    | Finished      | 
|-------------------|-------------------|---------------|
| Establish a UDP connection with a client. | in review | :heavy_check_mark:

- more coming soon...

### Unity Roadmap
This features wrappers for the project to make it easy to setup in Unity. Tasks to make the experience of utilizing this project more of a breeze can be found here.


- more coming soon...

# Collaboration
While I am still in the process of designing the structure of this project, I will not be actively accepting any collaborative efforts via pull requests. I am, however, open to being pointed in certain directions. Articles and documentation on specific issues are greatly appreciated. Even discussing it directly is welcome. If you're interested in that, feel free to join my [Discord](https://discord.makosai.com). You can discuss more about it in the #sustenet channel.
