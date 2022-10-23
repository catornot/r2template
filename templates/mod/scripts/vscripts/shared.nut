#if SERVER
global function Init_server
#else if CLIENT
global function Init_client
#endif

#if SERVER
void function Init_server()
{
    printt( "do smth" )
}

#else if CLIENT
void function Init_client()
{
    printt( "do smth" )
}
#endif