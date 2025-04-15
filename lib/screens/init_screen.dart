import 'package:flutter/material.dart';
import 'package:kobi/configs/configs.dart';
import 'package:kobi/screens/components/fade_image_widget.dart';
import '../cross.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'app_screen.dart';

class InitScreen extends StatefulWidget {
  const InitScreen({super.key});

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
    Navigator.of(context)
        .pushReplacement(MaterialPageRoute(builder: (_) => const AppScreen()));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: LayoutBuilder(
        builder: (BuildContext context, BoxConstraints constraints) {
          return Center(
            child: SizedBox(
              width: constraints.maxWidth / 2,
              height: constraints.maxHeight / 2,
              child: FadeImageWidget(
                child: Image.asset('lib/assets/startup.png'),
              ),
            ),
          );
        },
      ),
    );
  }
}
