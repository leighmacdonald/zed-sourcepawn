#include <sourcemod>

public Plugin myinfo = {
    name = "Hello World Plugin",
    author = "Example",
    description = "A basic hello world plugin",
    version = "1.0",
    url = "http://www.sourcemod.net/"
};

public void OnPluginStart() {
    // Register the command 'sm_hello'
    RegConsoleCmd("sm_hello", Command_Hello, "print hello world");

}

public Action Command_Hello(int client, int args) {
    ReplyToCommand(client, "[SM] Hello, World!");
    return Plugin_Handled;
}
