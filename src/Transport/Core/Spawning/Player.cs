using System.Numerics;

namespace Sustenet.Transport.Core.Spawning
{
    public class Player : Spawnee
    {
        public string username;

        public Player(int _id, string _username, Vector3 _spawnPosition)
        {
            id = _id;
            username = _username;
            position = _spawnPosition;
        }
    }
}
