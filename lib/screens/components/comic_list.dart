import 'package:flutter/material.dart';
import 'package:event/event.dart';
import '../../configs/comic_pager_type.dart';
import '../comic_info_screen.dart';
import 'package:kobi/configs/comic_grid_columns.dart';
import 'comic_card.dart';
import 'commons.dart';
import 'images.dart';

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

Widget comicListCard(BuildContext context, CommonComicInfo comic, int index, void Function(CommonComicInfo comic, int index)? onLongPress) {
  return GestureDetector(
    onTap: () {
      Navigator.of(context).push(MaterialPageRoute(
        builder: (BuildContext context) {
          return ComicInfoScreen(comicInfo: comic);
        },
      ));
    },
    onLongPress: onLongPress != null ? () => onLongPress(comic, index) : null,
    child: CommonComicCard(comic),
  );
}

List<Widget> comicListLines(
    BuildContext context, List<CommonComicInfo> records, void Function(CommonComicInfo comic, int index)? onLongPress) {
  List<Widget> lines = [];
  for (var i = 0; i < records.length; i++) {
    lines.add(comicListCard(context, records[i], i, onLongPress));
  }
  return lines;
}

List<Widget> comicGridLines(
    BuildContext context, List<CommonComicInfo> records, void Function(CommonComicInfo comic, int index)? onLongPress) {
  List<Widget> lines = [];
  final columns = currentComicGridColumns;
  
  for (var i = 0; i < records.length; i += columns) {
    List<Widget> rowChildren = [];
    for (var j = 0; j < columns; j++) {
      if (i + j < records.length) {
        rowChildren.add(
          Expanded(
            child: _buildGridCard(context, records[i + j], i + j, onLongPress),
          ),
        );
        if (j < columns - 1) {
          rowChildren.add(const SizedBox(width: 8));
        }
      } else {
        // 填充空白以保持布局
        rowChildren.add(const Expanded(child: SizedBox()));
        if (j < columns - 1) {
          rowChildren.add(const SizedBox(width: 8));
        }
      }
    }
    
    lines.add(
      Padding(
        padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 4.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: rowChildren,
        ),
      ),
    );
  }
  return lines;
}

Widget _buildGridCard(BuildContext context, CommonComicInfo comic, int index, void Function(CommonComicInfo comic, int index)? onLongPress) {
  return GestureDetector(
    onLongPress: onLongPress == null ? null : () => onLongPress(comic, index),
    onTap: () {
      Navigator.of(context).push(MaterialPageRoute(
        builder: (BuildContext context) {
          return ComicInfoScreen(comicInfo: comic);
        },
      ));
    },
    child: Container(
      decoration: BoxDecoration(
        color: Colors.grey[900],
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.2),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Stack(
            children: [
              AspectRatio(
                aspectRatio: 328 / 422, // 保持封面比例
                child: ClipRRect(
                  borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
                  child: LoadingCacheImage(
                    url: comic.cover,
                    useful: 'COMIC_COVER',
                    extendsFieldFirst: comic.pathWord,
                    fit: BoxFit.cover,
                  ),
                ),
              ),
              // 渐变遮罩
              Positioned.fill(
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
                    gradient: LinearGradient(
                      begin: Alignment.topCenter,
                      end: Alignment.bottomCenter,
                      colors: [
                        Colors.transparent,
                        Colors.black.withOpacity(0.3),
                      ],
                      stops: const [0.7, 1.0],
                    ),
                  ),
                ),
              ),
            ],
          ),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  comic.name,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 14,
                    fontWeight: FontWeight.w500,
                  ),
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                if (comic.author.isNotEmpty) ...[
                  const SizedBox(height: 4),
                  Text(
                    comic.author.map((a) => a.name).join(", "),
                    style: TextStyle(
                      color: Colors.grey[400],
                      fontSize: 12,
                    ),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
                if (comic.popular > 0) ...[
                  const SizedBox(height: 4),
                  Row(
                    children: [
                      Icon(
                        Icons.favorite,
                        size: 12,
                        color: Colors.pink[300],
                      ),
                      const SizedBox(width: 4),
                      Text(
                        comic.popular.toString(),
                        style: TextStyle(
                          color: Colors.grey[400],
                          fontSize: 12,
                        ),
                      ),
                    ],
                  ),
                ],
              ],
            ),
          ),
        ],
      ),
    ),
  );
}
