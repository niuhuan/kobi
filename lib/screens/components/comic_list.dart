import 'package:flutter/material.dart';
import '../comic_info_screen.dart';
import 'comic_card.dart';

// class ComicList extends StatelessWidget {
//   final List<CommonComicInfo> comics;
//   final ScrollController? scrollController;
//
//   const ComicList({Key? key, required this.comics, this.scrollController})
//       : super(key: key);
//
//   @override
//   Widget build(BuildContext context) {
//     return ListView.builder(
//       controller: scrollController,
//       padding: const EdgeInsets.all(0),
//       itemCount: comics.length,
//       itemBuilder: (context, index) {
//         return;
//       },
//     );
//   }
// }

List<Widget> comicListLines(
  BuildContext context,
  List<CommonComicInfo> comics, [
  void Function(CommonComicInfo comic, int index)? onLongPress,
]) {
  List<Widget> lines = [];
  for (var i = 0; i < comics.length; i++) {
    final value = comics[i];
    lines.add(GestureDetector(
      onTap: () {
        Navigator.of(context).push(MaterialPageRoute(
          builder: (BuildContext context) {
            return ComicInfoScreen(comicInfo: value);
          },
        ));
      },
      onLongPress: onLongPress != null ? () => onLongPress(value, i) : null,
      child: CommonComicCard(value),
    ));
  }
  return lines;
}

