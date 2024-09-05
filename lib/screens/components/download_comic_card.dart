import 'package:flutter/material.dart';

import '../../commons.dart';
import '../../src/rust/udto.dart';
import 'images.dart';

class DownloadComicCard extends StatelessWidget {
  final UIDownloadComic comic;

  const DownloadComicCard(this.comic, {super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(10),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: Colors.grey.shade400,
            width: .5,
          ),
        ),
      ),
      child: Row(
        children: [
          ClipRRect(
            borderRadius: const BorderRadius.all(Radius.circular(5)),
            child: LoadingCacheImage(
              url: comic.cover,
              width: 328 / 4,
              height: 422 / 4,
              useful: 'COMIC_COVER',
              extendsFieldFirst: comic.pathWord,
              fit: BoxFit.cover,
            ),
          ),
          Container(
            width: 10,
          ),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  comic.name + "\n",
                  style: const TextStyle(
                    fontSize: 16,
                  ),
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                ),
                Container(
                  height: 5,
                ),
                Text(
                  stringAuthors(comic.author).map((e) => e.name).join(','),
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.red.shade300,
                  ),
                ),
                Container(
                  height: 5,
                ),
                Row(children: [
                  Icon(
                    Icons.download,
                    size: 15,
                    color: Colors.grey.shade400,
                  ),
                  Text(
                    "${comic.imageCountSuccess}/${comic.imageCount}",
                    style: TextStyle(
                      fontSize: 13,
                      color: Colors.grey.shade400,
                    ),
                  ),
                  Expanded(child: Container()),
                  _flag(),
                ]),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _flag() {
    if (comic.downloadStatus == 0) {
      return Text(
        "下载中",
        style: TextStyle(
          color: Colors.blue,
        ),
      );
    }
    if (comic.downloadStatus == 1) {
      return Text(
        "完成",
        style: TextStyle(
          color: Colors.green,
        ),
      );
    }
    if (comic.downloadStatus == 2) {
      return Text(
        "失败",
        style: TextStyle(
          color: Colors.red,
        ),
      );
    }
    if (comic.downloadStatus == 3) {
      return Text(
        "删除",
        style: TextStyle(
          color: Colors.red,
        ),
      );
    }
    return Container();
  }
}
