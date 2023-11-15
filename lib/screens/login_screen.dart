
import 'package:flutter/material.dart';
import 'package:kobi/configs/login.dart';

import '../ffi.io.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({Key? key}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  late String _username = "";
  late String _password = "";

  @override
  void initState() {
    super.initState();
    loginEvent.subscribe(_setState);
    _loadProperties();
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  Future _loadProperties() async {
    var username = await api.loadProperty(k: "username");
    var password = await api.loadProperty(k: "password");
    setState(() {
      _username = username;
      _password = password;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold();
  }

}