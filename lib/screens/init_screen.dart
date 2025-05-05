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
      body: ConstrainedBox(
        constraints: const BoxConstraints.expand(),
        child: LayoutBuilder(
          builder: (BuildContext context, BoxConstraints constraints) {
            var width = 1024;
            var height = 1536;
            var min = constraints.maxWidth > constraints.maxHeight
                ? constraints.maxHeight
                : constraints.maxWidth;
            var newHeight = min;
            var newWidth = min * (width / height);
            return Center(
              child: ShaderMask(
                shaderCallback: (Rect bounds) {
                  return const LinearGradient(
                    begin: Alignment.topCenter,
                    end: Alignment.bottomCenter,
                    colors: [
                      Colors.black,
                      Colors.black,
                      Colors.transparent,
                    ],
                    stops: [0.0, 0.95, 1.0],
                  ).createShader(bounds);
                },
                blendMode: BlendMode.dstIn,
                child: Image.asset(
                  "lib/assets/startup.png",
                  width: newWidth,
                  height: newHeight,
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
