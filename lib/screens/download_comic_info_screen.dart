import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:kobi/commons.dart';
import 'package:kobi/screens/components/download_comic_card.dart';

import '../bridge_generated.dart';
import '../ffi.io.dart';
import 'comic_reader_screen.dart';
import 'components/images.dart';
import 'components/router.dart';

class DownloadComicInfoScreen extends StatefulWidget {
  final UIDownloadComic comic;

  const DownloadComicInfoScreen(this.comic, {Key? key}) : super(key: key);

  @override
  _DownloadComicInfoScreenState createState() =>
      _DownloadComicInfoScreenState();
}

class _DownloadComicInfoScreenState extends State<DownloadComicInfoScreen>
    with RouteAware {
  final _scrollController = ScrollController();
  double _scrollOffset = 0;
  late Future _fetchFuture = fetch();
  late Map<Group, List<UIComicChapter>> _gcMap;
  late UIComicQuery _query;
  late UIViewLog? _viewLog;

  @override
  void initState() {
    _scrollController.addListener(_onScroll);
    super.initState();
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
    routeObserver.unsubscribe(this);
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    routeObserver.subscribe(this, ModalRoute.of(context)!);
  }

  @override
  void didPopNext() {
    _loadViewLog();
  }

  static const _chapterLimit = 100;
  List<UIDownloadComicChapter> _dcs = [];

  Future fetch() async {
    final Map<Group, List<UIComicChapter>> gcMap = {};
    final groups =
        await api.downloadComicGroups(comicPathWord: widget.comic.pathWord);
    final chapters =
        await api.downloadComicChapters(comicPathWord: widget.comic.pathWord);
    _dcs = chapters;
    List<GC> gcs = [];
    for (var group in groups) {
      gcs.add(GC(
        group.groupPathWord,
        Group(
            count: group.count,
            name: group.name,
            pathWord: group.groupPathWord),
        [],
      ));
    }
    for (var chapter in chapters) {
      for (var gc in gcs) {
        if (gc.gid == chapter.groupPathWord) {
          gc.chapters.add(UIComicChapter(
            comicId: chapter.comicId,
            comicPathWord: chapter.comicPathWord,
            count: chapter.count,
            datetimeCreated: chapter.datetimeCreated,
            groupPathWord: chapter.groupPathWord,
            imgType: chapter.imgType,
            index: chapter.index,
            name: chapter.name,
            news: chapter.news,
            next: chapter.next,
            ordered: chapter.ordered,
            prev: chapter.prev,
            size: chapter.size,
            typeField: chapter.typeField,
            uuid: chapter.uuid,
          ));
          break;
        }
      }
    }
    for (var gc in gcs) {
      gc.chapters.sort((a, b) => a.index.compareTo(b.index));
      gcMap[gc.group] = gc.chapters;
    }
    final query = UIComicQuery(
        isLock: false, isLogin: false, isMobileBind: false, isVip: false);
    final viewLog = await api.findComicViewLog(pathWord: widget.comic.pathWord);
    _gcMap = gcMap;
    _query = query;
    _viewLog = viewLog;
    // async
    api.viewComicInfo(
      comicPathWord: widget.comic.pathWord,
      comicName: widget.comic.name,
      comicAuthors: stringAuthors(widget.comic.author),
      comicCover: widget.comic.cover,
    );
  }

  _loadViewLog() async {
    final viewLog = await api.findComicViewLog(pathWord: widget.comic.pathWord);
    setState(() {
      _viewLog = viewLog;
    });
  }

  void _onScroll() {
    setState(() {
      _scrollOffset = _scrollController.offset;
    });
  }

  static const _appHiddenStart = 50.0;
  static const _appHiddenEnd = 150.0;

  double get _appbarOpacity => _scrollOffset < _appHiddenStart
      ? 1.0
      : _scrollOffset > _appHiddenEnd
          ? 0.0
          : (_appHiddenEnd - _scrollOffset) / (_appHiddenEnd - _appHiddenStart);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(children: [
        ..._background(),
        _comicInfo(),
        _titleBar(),
        _floatButtons(),
      ]),
    );
  }

  List<Widget> _background() {
    return [
      Opacity(
        opacity: .25,
        child: LayoutBuilder(
          builder: (
            BuildContext context,
            BoxConstraints constraints,
          ) {
            return ShaderMask(
              shaderCallback: (rect) {
                return const LinearGradient(
                  begin: Alignment.topCenter,
                  end: Alignment.bottomCenter,
                  colors: [
                    Colors.black,
                    Colors.transparent,
                  ],
                ).createShader(
                  Rect.fromLTRB(0, 0, rect.width, rect.height),
                );
              },
              blendMode: BlendMode.dstIn,
              child: LoadingCacheImage(
                url: widget.comic.cover,
                width: constraints.maxWidth,
                height: constraints.maxHeight / 3,
                useful: 'COMIC_COVER',
                extendsFieldFirst: widget.comic.pathWord,
                fit: BoxFit.fill,
              ),
            );
          },
        ),
      ),
      Positioned.fromRect(
        rect: Rect.largest,
        child: BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 30, sigmaY: 30),
          child: Container(),
        ),
      ),
    ];
  }

  Widget _comicInfo() {
    return FutureBuilder(
      future: _fetchFuture,
      builder: (BuildContext context, AsyncSnapshot snapshot) {
        if (snapshot.hasError) {
          print("${snapshot.error}");
          print("${snapshot.stackTrace}");
          return const Center(
            child: Text('加载失败'),
          );
        }
        if (snapshot.connectionState != ConnectionState.done) {
          return const Center(
            child: CircularProgressIndicator(),
          );
        }
        return _comicInfoLoaded();
      },
    );
  }

  Widget _comicInfoLoaded() {
    return ListView(
      controller: _scrollController,
      children: [
        // 站位APP-BAR
        AppBar(
          automaticallyImplyLeading: false,
          elevation: 0,
          backgroundColor: Colors.transparent,
        ),
        DownloadComicCard(widget.comic),
        Container(
          padding: const EdgeInsets.all(10),
          child: _brief(widget.comic.brief),
        ),
        const Divider(),
        ..._continueAndRestart(),
        ..._chapters(),
        const Divider(),
        SafeArea(child: Container()),
      ],
    );
  }

  Widget _titleBar() {
    final theme = Theme.of(context);
    return Opacity(
      opacity: _appbarOpacity,
      child: Column(
        children: [
          AppBar(
            centerTitle: true,
            leading: IconButton(
              icon: const Icon(
                Icons.arrow_back,
                color: Colors.transparent,
              ),
              onPressed: () {},
            ),
            title: Text(
              widget.comic.name,
            ),
            foregroundColor: theme.textTheme.bodyMedium?.color,
            backgroundColor: Colors.transparent,
            elevation: .0,
            actions: const [],
          ),
        ],
      ),
    );
  }

  Widget _floatButtons() {
    final theme = Theme.of(context);
    return Column(
      children: [
        AppBar(
          centerTitle: true,
          leading: IconButton(
            icon: Icon(
              Icons.arrow_back,
              color: theme.textTheme.bodyMedium?.color,
              shadows: [
                Shadow(
                  color: Colors.white.withOpacity(1 - _appbarOpacity),
                  offset: const Offset(0, 0),
                  blurRadius: 5,
                ),
              ],
            ),
            onPressed: () {
              Navigator.of(context).pop();
            },
          ),
          backgroundColor: Colors.transparent,
          elevation: .0,
          actions: [],
        ),
      ],
    );
  }

  Widget _brief(String text) {
    return Text(text);
  }

  List<Widget> _continueAndRestart() {
    List<Widget> list = [];
    if (_viewLog != null && _viewLog!.chapterUuid.isNotEmpty) {
      list.add(Container(
        padding: const EdgeInsets.only(
          left: 30,
          right: 30,
          top: 5,
          bottom: 5,
        ),
        child: Row(
          children: [
            Expanded(
              child: MaterialButton(
                elevation: 0,
                color: Colors.grey.shade500.withOpacity(.3),
                textColor: Theme.of(context).textTheme.bodyMedium?.color,
                child: Text(
                    "继续阅读 : ${_viewLog!.chapterName} (${_viewLog!.pageRank + 1})"),
                onPressed: _continueRead,
              ),
            ),
          ],
        ),
      ));
    }
    {
      list.add(Container(
        padding: const EdgeInsets.only(
          left: 30,
          right: 30,
          top: 5,
          bottom: 5,
        ),
        child: Row(
          children: [
            Expanded(
              child: MaterialButton(
                elevation: 0,
                color: Colors.grey.shade500.withOpacity(.3),
                textColor: Theme.of(context).textTheme.bodyMedium?.color,
                child: const Text("从头开始"),
                onPressed: _startRead,
              ),
            ),
          ],
        ),
      ));
    }
    return list;
  }

  List<Widget> _chapters() {
    if (_gcMap.length == 1) {
      for (var e in _gcMap.entries) {
        return _groupChapters(e.value);
      }
    } else {
      List<Widget> _result = [];
      for (var e in _gcMap.entries) {
        _result.addAll(_groupTitle(e.key));
        _result.addAll(_groupChapters(e.value));
      }
      return _result;
    }
    return [];
  }

  List<Widget> _groupTitle(Group key) {
    return [
      Center(
        child: Text("----  ${key.name}  ----",
            style: const TextStyle(
              fontSize: 14,
              color: Colors.grey,
              height: 2,
            )),
      ),
    ];
  }

  List<Widget> _groupChapters(List<UIComicChapter> value) {
    return [
      Container(
        margin: const EdgeInsets.only(
          left: 10,
          right: 10,
        ),
        child: Wrap(
          spacing: 10,
          runSpacing: 3,
          alignment: WrapAlignment.center,
          children: value.map(_buildChapter).toList(),
        ),
      ),
    ];
  }

  Widget _buildChapter(UIComicChapter c) {
    return GestureDetector(
      onTap: () {
        _goReader(c, 0);
      },
      child: Card(
        elevation: 0.3,
        child: Container(
          padding: const EdgeInsets.only(
            left: 10,
            right: 10,
            top: 5,
            bottom: 5,
          ),
          child: Text(c.name),
        ),
      ),
    );
  }

  void _goReader(UIComicChapter c, int initRank) {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (context) => ComicReaderScreen(
          comic: UIComic(
            author: [],
            b404: widget.comic.b404,
            bHidden: widget.comic.bHidden,
            ban: widget.comic.ban,
            brief: widget.comic.brief,
            closeComment: widget.comic.closeComment,
            closeRoast: widget.comic.closeRoast,
            cover: widget.comic.cover,
            datetimeUpdated: widget.comic.datetimeUpdated,
            freeType: stringClassifyItem(widget.comic.freeType),
            imgType: widget.comic.imgType,
            lastChapter: const LastChapter(name: "", uuid: ""),
            name: widget.comic.name,
            pathWord: widget.comic.pathWord,
            popular: widget.comic.popular,
            reclass: stringClassifyItem(widget.comic.reclass),
            region: stringClassifyItem(widget.comic.region),
            restrict: stringClassifyItem(widget.comic.restrict),
            seoBaidu: widget.comic.seoBaidu,
            status: stringClassifyItem(widget.comic.status),
            theme: [],
            uuid: widget.comic.uuid,
            females: [],
            males: [],
          ),
          chapterUuid: c.uuid,
          initRank: initRank,
          loadChapter: (String comicPathWord, String chapterUuid) async {
            late UIDownloadComicChapter dc;
            for (var c in _dcs) {
              if (c.comicPathWord == comicPathWord && c.uuid == chapterUuid) {
                dc = c;
                break;
              }
            }
            final pages = await api.downloadComicPages(
              comicPathWord: comicPathWord,
              chapterUuid: chapterUuid,
            );
            return UIChapterAndContents(
              comicId: c.comicId,
              comicPathWord: c.comicPathWord,
              contents: pages.map((e) => ChapterImage(url: e.url)).toList(),
              count: c.count,
              datetimeCreated: c.datetimeCreated,
              groupPathWord: c.groupPathWord,
              imgType: c.imgType,
              index: c.index,
              isLong: false,
              name: c.name,
              news: c.news,
              ordered: c.ordered,
              size: c.size,
              typeField: c.typeField,
              uuid: c.uuid,
              words: Int64List.fromList(
                List<int>.generate(pages.length, (index) => index),
              ),
            );
          },
          groupChaptersMap: _gcMap,
        ),
      ),
    );
  }

  void _continueRead() {
    if (_viewLog == null) {
      return;
    }
    for (var e in _gcMap.entries) {
      for (var c in e.value) {
        if (c.uuid == _viewLog!.chapterUuid) {
          _goReader(
            c,
            _viewLog!.pageRank,
          );
          return;
        }
      }
    }
  }

  void _startRead() {
    for (var e in _gcMap.entries) {
      for (var c in e.value) {
        _goReader(
          c,
          0,
        );
        return;
      }
    }
  }
}

class GC {
  String gid;
  Group group;
  List<UIComicChapter> chapters;

  GC(this.gid, this.group, this.chapters);
}
