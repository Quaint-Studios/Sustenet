# Sustenet
A Zig, formerly C#, networking solution for the Godot Engine, Unreal Engine, and Unity3D. The primary focus is to enable scaling by allowing multiple servers to work together.
[![Sustenet (Zig)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-zig.yml/badge.svg)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-zig.yml) [![CodeFactor](https://www.codefactor.io/repository/github/quaint-studios/sustenet/badge)](https://www.codefactor.io/repository/github/quaint-studios/sustenet)

Note: This README is out of date since we're still migrating from C# to Zig. Things like solutions and project files no longer exist. Regardless, the layout should remain the same for everything.

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

# Building & Testing
Here's a little context on the structure. There are two solutions. `Sustenet` and `SustenetUnity`. The former generates an executable while the latter generates a library. `SustenetUnity` also excludes all files related to the master server.

Inside of the SustenetUnity.csproj file under PostBuild is a line that says `ImplementationPath`. That is the path you want the library to be automatically exported to. It's advised to change it to something valid. Other than that, everything should work as is.

## Testing with no GUI
You can run the Sustenet.exe by itself with the parameter --master (this is the default options, so you don't actually have to provide it) and --client in two separate programs. This will show you an example connection.

# Collaboration
While I am still in the process of designing the structure of this project, I will not be actively accepting any collaborative efforts via pull requests. I am, however, open to being pointed in certain directions. Articles and documentation on specific issues are greatly appreciated. Even discussing it directly is welcome. If you're interested in that, feel free to join my [Discord](https://discord.makosai.com). You can discuss more about it in the #sustenet channel.
