import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:kobi/screens/components/commons.dart';
import '../bridge_generated.dart';
import '../ffi.io.dart';
import 'components/comic_list.dart';
import 'components/comic_pager.dart';
import 'components/images.dart';

class BooksScreen extends StatelessWidget {
  const BooksScreen({super.key});

  @override
  Widget build(BuildContext context) {
    ThemeData theme = Theme.of(context);
    return DefaultTabController(
      length: 2,
      child: Column(
        children: [
          SafeArea(
            child: Container(),
            bottom: false,
          ),
          Container(
            height: 40,
            color: theme.colorScheme.secondary.withOpacity(.025),
            child: const TabBar(
              tabs: [
                Tab(text: '历史'),
                Tab(text: '下载'),
              ],
            ),
          ),
          const Expanded(
            child: TabBarView(
              children: [
                HistoryScreen(),
                DownloadsScreen(),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class HistoryScreen extends StatelessWidget {
  const HistoryScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return ComicPager(fetcher: (offset, limit) async {
      final result = await api.listComicViewLogs(offset: offset, limit: limit);
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) => CommonComicInfo(
                  author: _stringAuthors(e.comicAuthors),
                  cover: e.comicCover,
                  imgType: 1,
                  name: e.comicName,
                  pathWord: e.comicPathWord,
                  popular: 0,
                  males: [],
                  females: [],
                ))
            .toList(),
        total: result.total,
        limit: result.limit,
        offset: result.offset,
      );
    });
  }
}

class DownloadsScreen extends StatefulWidget {
  const DownloadsScreen({super.key});

  @override
  State<StatefulWidget> createState() => _DownloadsScreenState();
}

class _DownloadsScreenState extends State<DownloadsScreen> {

  List<UIDownloadComic> list = [];
  bool paused = false;

  _init() async {
    list = await api.downloadComics();
    paused = await api.downloadIsPause();
    setState(() {});
  }

  @override
  void initState() {
    _init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Container(
          padding: const EdgeInsets.only(
            left: 10,
            right: 10,
          ),
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
              Expanded(child: Container()),
              Text(paused ? "暂停中" : "下载中"),
              IconButton(
                onPressed: () async {
                  await api.downloadSetPause(pause: !paused);
                  await _init();
                },
                icon: Icon(paused ? Icons.play_arrow : Icons.pause),
              ),
              IconButton(
                onPressed: () async {
                  await api.resetFailDownloads();
                  defaultToast(context, "失败的任务已经重置");
                  await _init();
                },
                icon: const Icon(Icons.refresh),
              ),
            ],
          ),
        ),
        Expanded(
          child: ListView(
            padding: EdgeInsets.zero,
            children: [
              ...list.map((e) => DownloadComicCard(e)),
            ],
          ),
        ),
      ],
    );
  }
}

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
                  _stringAuthors(comic.author).map((e) => e.name).join(','),
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
    return Container();
  }
}

List<Author> _stringAuthors(String data) {
  return _mapAuthor(List.of(jsonDecode(data)).cast());
}

List<Author> _mapAuthor(List<Map> list) {
  List<Author> result = [];
  for (var value in list) {
    if (value['name'] != null && value['path_word'] != null) {
      result.add(Author(
        name: value['name'],
        pathWord: value['path_word'],
      ));
    }
  }
  return result;
}
