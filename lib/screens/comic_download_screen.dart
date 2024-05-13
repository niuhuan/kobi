import 'dart:convert';

import 'package:flutter/material.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'package:kobi/screens/components/commons.dart';
import 'package:kobi/screens/components/content_loading.dart';

import '../src/rust/copy_client/dtos.dart';
import '../src/rust/udto.dart';
import 'components/comic_card.dart';

class ComicDownloadScreen extends StatefulWidget {
  final UIComic comic;
  final Map<Group, List<UIComicChapter>> groupChaptersMap;

  const ComicDownloadScreen({
    Key? key,
    required this.comic,
    required this.groupChaptersMap,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() {
    return _ComicDownloadScreenState();
  }
}

class _ComicDownloadScreenState extends State<ComicDownloadScreen> {
  late Future _future;
  late List<String> _inDownloadedChapters;
  final List<String> _selectedChapters = [];

  _init() async {
    _inDownloadedChapters =
        await api.inDownloadChapterUuid(comicPathWord: widget.comic.pathWord);
  }

  @override
  void initState() {
    _future = _init();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("下载"),
      ),
      body: FutureBuilder(
        future: _future,
        builder: (
          BuildContext context,
          AsyncSnapshot snapshot,
        ) {
          if (snapshot.hasError) {
            return Center(
              child: Text(snapshot.error.toString()),
            );
          }
          if (snapshot.connectionState == ConnectionState.done) {
            return _buildBody(context);
          }
          return const ContentLoading();
        },
      ),
    );
  }

  Widget _buildBody(BuildContext context) {
    return ListView(children: [
      CommonComicCard(CommonComicInfo(
        author: widget.comic.author,
        cover: widget.comic.cover,
        imgType: widget.comic.imgType,
        name: widget.comic.name,
        pathWord: widget.comic.pathWord,
        popular: widget.comic.popular,
        females: widget.comic.females,
        males: widget.comic.males,
      )),
      _buildButtons(),
      ..._buildGroups(widget.groupChaptersMap),
    ]);
  }

  Widget _buildButtons() {
    var theme = Theme.of(context);
    return Container(
      padding: const EdgeInsets.all(5),
      decoration: BoxDecoration(
        border: Border(
          bottom: BorderSide(
            color: Colors.grey.shade200,
          ),
        ),
      ),
      child: Wrap(
        spacing: 10,
        runSpacing: 10,
        alignment: WrapAlignment.spaceAround,
        children: [
          MaterialButton(
            color: theme.colorScheme.secondary,
            textColor: Colors.white,
            onPressed: _selectAll,
            child: const Text('全选'),
          ),
          MaterialButton(
            color: theme.colorScheme.secondary,
            textColor: Colors.white,
            onPressed: _download,
            child: const Text('确定下载'),
          ),
        ],
      ),
    );
  }

  Color _colorOfEp(UIComicChapter e) {
    if (_inDownloadedChapters.contains(e.uuid)) {
      return Colors.grey.shade300;
    }
    if (_selectedChapters.contains(e.uuid)) {
      return Colors.blueGrey.shade300;
    }
    return Colors.grey.shade200;
  }

  Icon _iconOfEp(UIComicChapter e) {
    if (_inDownloadedChapters.contains(e.uuid)) {
      return const Icon(Icons.download_rounded, color: Colors.black);
    }
    if (_selectedChapters.contains(e.uuid)) {
      return const Icon(Icons.check_box, color: Colors.black);
    }
    return const Icon(Icons.check_box_outline_blank, color: Colors.black);
  }

  void _clickOfEp(UIComicChapter e) {
    if (_inDownloadedChapters.contains(e.uuid)) {
      return;
    }
    if (_selectedChapters.contains(e.uuid)) {
      setState(() {
        _selectedChapters.remove(e.uuid);
      });
    } else {
      setState(() {
        _selectedChapters.add(e.uuid);
      });
    }
  }

  List<Widget> _buildGroups(Map<Group, List<UIComicChapter>> groupChaptersMap) {
    if (groupChaptersMap.length == 1) {
      return [_buildChapters(groupChaptersMap.values.first)];
    }
    List<Widget> result = [];
    for (var entry in groupChaptersMap.entries) {
      result.add(_buildGroupTitle(entry.key));
      result.add(_buildChapters(entry.value));
    }
    return result;
  }

  Widget _buildGroupTitle(Group g) {
    return Center(
      child: Text("----  ${g.name}  ----",
          style: const TextStyle(
            fontSize: 14,
            color: Colors.grey,
            height: 2,
          )),
    );
  }

  Widget _buildChapters(List<UIComicChapter> chapters) {
    return Wrap(
      alignment: WrapAlignment.spaceAround,
      runSpacing: 10,
      spacing: 10,
      children: [
        ...chapters.map((e) {
          return Container(
            padding: const EdgeInsets.all(5),
            child: MaterialButton(
              onPressed: () {
                _clickOfEp(e);
              },
              color: _colorOfEp(e),
              child: Row(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  _iconOfEp(e),
                  Container(
                    width: 10,
                  ),
                  Text(
                    e.name,
                    style: const TextStyle(color: Colors.black),
                  ),
                ],
              ),
            ),
          );
        }),
      ],
    );
  }

  _selectAll() {
    List<String> uuidList = [];
    for (var cList in widget.groupChaptersMap.values) {
      for (var value in cList) {
        uuidList.add(value.uuid);
      }
    }
    for (var value in _inDownloadedChapters) {
      uuidList.remove(value);
    }
    if (uuidList.length == _selectedChapters.length) {
      setState(() {
        _selectedChapters.clear();
      });
    } else {
      setState(() {
        _selectedChapters.clear();
        _selectedChapters.addAll(uuidList);
      });
    }
  }

  _download() async {
    if (_selectedChapters.isEmpty) {
      defaultToast(context, "请选择章节");
      return;
    }
    try {
      int gr = 0;
      List<UIComicChapter> chapters = [];
      for (var cList in widget.groupChaptersMap.values) {
        for (var value in cList) {
          if (_selectedChapters.contains(value.uuid)) {
            chapters.add(value);
          }
        }
      }
      await api.appendDownload(
        data: UIQueryDownloadComic(
          pathWord: widget.comic.pathWord,
          author: jsonEncode(widget.comic.author
              .map((e) => {
                    "path_word": e.pathWord,
                    "name": e.name,
                    "alias": e.alias,
                  })
              .toList()),
          b404: widget.comic.b404,
          bHidden: widget.comic.bHidden,
          ban: widget.comic.ban,
          brief: widget.comic.brief,
          closeComment: widget.comic.closeComment,
          closeRoast: widget.comic.closeRoast,
          cover: widget.comic.cover,
          datetimeUpdated: widget.comic.datetimeUpdated,
          females: jsonEncode(widget.comic.females
              .map((e) => {
                    "name": e.name,
                    "gender": e.gender,
                    "path_word": e.pathWord,
                  })
              .toList()),
          freeType: jsonEncode({
            "value": widget.comic.freeType.value,
            "display": widget.comic.freeType.display,
          }),
          imgType: widget.comic.imgType,
          males: jsonEncode(widget.comic.males
              .map((e) => {
                    "name": e.name,
                    "gender": e.gender,
                    "path_word": e.pathWord,
                  })
              .toList()),
          name: widget.comic.name,
          popular: widget.comic.popular,
          reclass: jsonEncode({
            "value": widget.comic.reclass.value,
            "display": widget.comic.reclass.display,
          }),
          region: jsonEncode({
            "value": widget.comic.region.value,
            "display": widget.comic.region.display,
          }),
          restrict1: jsonEncode({
            "value": widget.comic.restrict.value,
            "display": widget.comic.restrict.display,
          }),
          seoBaidu: widget.comic.seoBaidu,
          status: jsonEncode({
            "value": widget.comic.status.value,
            "display": widget.comic.status.display,
          }),
          theme: jsonEncode(widget.comic.theme
              .map((e) => {
                    "path_word": e.pathWord,
                    "name": e.name,
                  })
              .toList()),
          uuid: widget.comic.uuid,
          groups: widget.groupChaptersMap.keys
              .map((e) => UIQueryDownloadComicGroup(
                    comicPathWord: widget.comic.pathWord,
                    groupPathWord: e.pathWord,
                    name: e.name,
                    count: e.count,
                    groupRank: gr++,
                  ))
              .toList(),
          chapters: chapters
              .map((e) => UIQueryDownloadComicChapter(
                    comicPathWord: e.comicPathWord,
                    uuid: e.uuid,
                    comicId: e.comicId,
                    count: e.count,
                    datetimeCreated: e.datetimeCreated,
                    groupPathWord: e.groupPathWord,
                    imgType: e.imgType,
                    index: e.index,
                    isLong: false,
                    name: e.name,
                    news: e.news,
                    next: e.next,
                    ordered: e.ordered,
                    prev: e.prev,
                    size: e.size,
                    typeField: e.typeField,
                  ))
              .toList(),
        ),
      );
      defaultToast(context, "已经添加到下载");
      Navigator.of(context).pop();
    } catch (e, s) {
      print("$e\n$s");
      defaultToast(context, "下载失败");
      return;
    }
  }
}
