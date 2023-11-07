import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:kobi/configs/configs.dart';
import '../cross.dart';
import '../ffi.io.dart';
import 'app_screen.dart';

class InitScreen extends StatefulWidget {
  const InitScreen();

  @override
  _InitScreenState createState() => _InitScreenState();

}

class _InitScreenState extends State<InitScreen> {
  @override
  void initState() {
    super.initState();
    init();
  }

  Future<void> init() async {
    await api.init(root: await cross.root());
    await initConfigs();
    RendererBinding.instance.addPostFrameCallback((_) {
      Navigator.of(context).push(MaterialPageRoute(builder: (_) => const AppScreen()));
    });
  }

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Text('Init Screen'),
      ),
    );
  }
}