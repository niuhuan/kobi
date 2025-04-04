import 'package:flutter/material.dart';

class ContentLoading extends StatelessWidget {
  final String label;
  final bool sq;

  const ContentLoading({Key? key, this.label = "加载中", this.sq = false}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (BuildContext context, BoxConstraints constraints) {
        var width = constraints.maxWidth;
        var height = constraints.maxHeight;
        if (sq) {
          height = width;
        }
        var min = width < height ? width : height;
        var theme = Theme.of(context);
        return SizedBox(
          width: width,
          height: height,
          child: Center(
            child: Column(
              children: [
                Expanded(child: Container()),
                SizedBox(
                  width: min / 2,
                  height: min / 2,
                  child: CircularProgressIndicator(
                    color: theme.colorScheme.secondary,
                    backgroundColor: Colors.grey[100],
                  ),
                ),
                Container(height: min / 10),
                Text(label, style: TextStyle(fontSize: min / 15)),
                Expanded(child: Container()),
              ],
            ),
          ),
        );
      },
    );
  }
}
