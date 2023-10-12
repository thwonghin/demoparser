fn main() {
    protobuf_codegen::Codegen::new()
        // Use `protoc` parser, optional.
        .protoc()
        // Use `protoc-bin-vendored` bundled protoc command, optional.
        //.protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["Protobufs/csgo/"])
        // Inputs must reside in some of include paths.
        .input("Protobufs/csgo/demo.proto")
        .input("Protobufs/csgo/steamdatagram_messages_auth.proto")
        .input("Protobufs/csgo/cstrike15_usermessages.proto")
        .input("Protobufs/csgo/econ_gcmessages.proto")
        .input("Protobufs/csgo/steamdatagram_messages_sdr.proto")
        .input("Protobufs/csgo/demo.proto")
        .input("Protobufs/csgo/enums_clientserver.proto")
        .input("Protobufs/csgo/steammessages_oauth.steamworkssdk.proto")
        .input("Protobufs/csgo/gameevents.proto")
        .input("Protobufs/csgo/cstrike15_gcmessages.proto")
        .input("Protobufs/csgo/networksystem_protomessages.proto")
        .input("Protobufs/csgo/c_peer2peer_netmessages.proto")
        .input("Protobufs/csgo/usermessages.proto")
        .input("Protobufs/csgo/usercmd.proto")
        .input("Protobufs/csgo/steamnetworkingsockets_messages_certs.proto")
        .input("Protobufs/csgo/gcsystemmsgs.proto")
        .input("Protobufs/csgo/networkbasetypes.proto")
        .input("Protobufs/csgo/steammessages_helprequest.steamworkssdk.proto")
        .input("Protobufs/csgo/steammessages_publishedfile.steamworkssdk.proto")
        .input("Protobufs/csgo/steammessages_publishedfile.steamworkssdk.proto")
        .input("Protobufs/csgo/uifontfile_format.proto")
        .input("Protobufs/csgo/connectionless_netmessages.proto")
        .input("Protobufs/csgo/engine_gcmessages.proto")
        .input("Protobufs/csgo/steammessages_unified_base.steamworkssdk.proto")
        .input("Protobufs/csgo/steammessages.proto")
        .input("Protobufs/csgo/netmessages.proto")
        .input("Protobufs/csgo/network_connection.proto")
        .input("Protobufs/csgo/steammessages_gamenetworkingui.proto")
        .input("Protobufs/csgo/steammessages_cloud.steamworkssdk.proto")
        .input("Protobufs/csgo/steamnetworkingsockets_messages.proto")
        .input("Protobufs/csgo/te.proto")
        .input("Protobufs/csgo/steammessages_steamlearn.steamworkssdk.proto")
        .input("Protobufs/csgo/cs_usercmd.proto")
        .input("Protobufs/csgo/valveextensions.proto")
        .input("Protobufs/csgo/clientmessages.proto")
        .input("Protobufs/csgo/steamnetworkingsockets_messages_udp.proto")
        .input("Protobufs/csgo/cs_gameevents.proto")
        .input("Protobufs/csgo/fatdemo.proto")
        .input("Protobufs/csgo/gcsdk_gcmessages.proto")
        .input("Protobufs/csgo/steammessages_player.steamworkssdk.proto")
        .out_dir("src/")
        .run_from_script();
}
