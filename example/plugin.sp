#include <sourcemod>
#include <tf2>
#include <tf2_stocks>

enum struct ES {
    int a
    char b[244]
}

public Plugin myinfo = {
    name = "Hello World Plugin",
    author = "Example",
    description = "A basic hello world plugin",
    version = "1.0",
    url = "http://www.sourcemod.net/"
};

public void OnPluginStart() {
    RegConsoleCmd("sm_hello", Command_Hello, "print hello world");
    int x = view_as<int>("1");
    float y = 100.0;
    bool z = true;
    char playerName[32] = "asdf";
    PrintToChat(client, "\x01\x04[SM] Hello %s! Count: %d, Speed: %.2f", playerName, g_iCount, g_fSpeed);
}

public Action Command_Hello(int client, int args) {
    ReplyToCommand(client, "[SM] Hello, World!");
    return Plugin_Handled;
}

methodmap BlaMap < Handle {
    // Constructor
    public BlaMap() {
        return view_as<BlaMap>(CreateTrie());
    }

    public void SetValue(const char[] key, int value) {
        SetTrieValue(view_as<Handle>(this), key, value);
    }
}
