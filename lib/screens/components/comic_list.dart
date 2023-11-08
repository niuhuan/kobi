import 'package:flutter/material.dart';

import '../../bridge_generated.dart';
import '../comic_info_screen.dart';
import 'images.dart';

class ComicList extends StatelessWidget {
  final List<CommonComicInfo> comics;

  const ComicList({Key? key, required this.comics}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      padding: const EdgeInsets.all(0),
      itemCount: comics.length,
      itemBuilder: (context, index) {
        return GestureDetector(
          onTap: () {
            Navigator.of(context).push(MaterialPageRoute(
              builder: (BuildContext context) {
                return ComicInfoScreen(comicInfo: comics[index]);
              },
            ));
          },
          child: CommonComicCard(comics[index]),
        );
      },
    );
  }
}

class CommonComicCard extends StatelessWidget {
  final CommonComicInfo comic;

  const CommonComicCard(this.comic, {super.key});

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
                  comic.author.map((e) => e.name).join(','),
                  style: TextStyle(
                    fontSize: 12,
                    color: Colors.red.shade300,
                  ),
                ),
                Container(
                  height: 5,
                ),
                Text.rich(TextSpan(children: [
                  const WidgetSpan(
                    child: Icon(
                      Icons.local_fire_department,
                      size: 16,
                      color: Colors.grey,
                    ),
                  ),
                  TextSpan(
                    text: comic.popular.toString(),
                    style: const TextStyle(
                      fontSize: 12,
                      color: Colors.grey,
                    ),
                  ),
                ])),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class CommonComicInfo {
  final List<Author> author;
  final String cover;
  final int imgType;
  final String name;
  final String pathWord;
  final int popular;

  const CommonComicInfo({
    required this.author,
    required this.cover,
    required this.imgType,
    required this.name,
    required this.pathWord,
    required this.popular,
  });
}
