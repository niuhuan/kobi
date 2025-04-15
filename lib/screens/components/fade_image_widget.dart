import 'package:flutter/material.dart';

class FadeImageWidget extends StatelessWidget {

  final Widget child;

  const FadeImageWidget({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return ShaderMask(
      shaderCallback: (Rect bounds) {
        return const RadialGradient(
          center: Alignment.center,
          radius: 0.8,
          colors: [
            Colors.black,
            Colors.black,
            Colors.transparent,
          ],
          stops: [0.7, 0.95, 1.0], // 渐变范围
        ).createShader(bounds);
      },
      blendMode: BlendMode.dstIn, // 混合模式，保留透明部分
      child: child,
    );
  }
}