import 'dart:developer';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:kobi/bridge_generated.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/ffi.io.dart';
import 'package:kobi/screens/components/commons.dart';

import 'comic_download_screen.dart';
import 'comic_reader_screen.dart';
import 'components/comic_card.dart';
import 'components/images.dart';
import 'components/router.dart';

class ComicInfoScreen extends StatefulWidget {
  final CommonComicInfo comicInfo;

  const ComicInfoScreen({Key? key, required this.comicInfo}) : super(key: key);

  @override
  _ComicInfoScreenState createState() => _ComicInfoScreenState();
}

class _ComicInfoScreenState extends State<ComicInfoScreen> with RouteAware {
  final _scrollController = ScrollController();
  double _scrollOffset = 0;
  late Future _fetchFuture = fetch();
  late UIComicData _comic;
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

  Future fetch() async {
    final comic = await api.comic(pathWord: widget.comicInfo.pathWord);
    final Map<Group, List<UIComicChapter>> gcMap = {};
    for (var group in comic.groups) {
      var offset = 0;
      List<UIComicChapter> cList = [];
      while (true) {
        final response = await api.comicChapters(
          comicPathWord: widget.comicInfo.pathWord,
          groupPathWord: group.pathWord,
          offset: offset,
          limit: _chapterLimit,
        );
        cList.addAll(response.list);
        offset += _chapterLimit;
        if (response.total <= offset) {
          break;
        }
      }
      gcMap[group] = cList;
    }
    final query = await api.comicQuery(pathWord: widget.comicInfo.pathWord);
    final viewLog =
        await api.findComicViewLog(pathWord: widget.comicInfo.pathWord);
    _comic = comic;
    _gcMap = gcMap;
    _query = query;
    _viewLog = viewLog;
    // async
    api.viewComicInfo(
      comicPathWord: comic.comic.pathWord,
      comicName: comic.comic.name,
      comicAuthors: comic.comic.author,
      comicCover: comic.comic.cover,
    );
  }

  _loadViewLog() async {
    final viewLog =
        await api.findComicViewLog(pathWord: widget.comicInfo.pathWord);
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

  bool _collecting = false;

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
                url: widget.comicInfo.cover,
                width: constraints.maxWidth,
                height: constraints.maxHeight / 3,
                useful: 'COMIC_COVER',
                extendsFieldFirst: widget.comicInfo.pathWord,
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
        ComicInfoCard(_comic.comic),
        Container(
          padding: const EdgeInsets.all(10),
          child: _brief(_comic.comic.brief),
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
              widget.comicInfo.name,
            ),
            foregroundColor: theme.textTheme.bodyMedium?.color,
            backgroundColor: Colors.transparent,
            elevation: .0,
            actions: [
              IconButton(
                onPressed: () async {},
                icon: Icon(Icons.add),
                color: Colors.transparent,
              ),
              IconButton(
                onPressed: () async {},
                icon: Icon(Icons.add),
                color: Colors.transparent,
              ),
            ],
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
          actions: [
            FutureBuilder(
              future: _fetchFuture,
              builder: (
                BuildContext context,
                AsyncSnapshot<dynamic> snapshot,
              ) {
                if (snapshot.connectionState == ConnectionState.done) {
                  if (loginState.state != 1) {
                    return Container();
                  }
                  if (_collecting) {
                    return IconButton(
                      onPressed: () {},
                      icon: const Icon(Icons.refresh),
                    );
                  }
                  return IconButton(
                    onPressed: () async {
                      if (_collecting) {
                        return;
                      }
                      setState(() {});
                      final collected = (_query.collect ?? 0) > 0;
                      try {
                        _collecting = true;
                        await api.collectToAccount(
                          comicId: _comic.comic.uuid,
                          isCollect: !collected,
                          comicPathWord: _comic.comic.pathWord,
                        );
                        final result = collected ? 0 : 1;
                        _query = UIComicQuery(
                          collect: result,
                          isLock: _query.isLock,
                          isLogin: _query.isLogin,
                          isMobileBind: _query.isMobileBind,
                          isVip: _query.isVip,
                        );
                        defaultToast(context, "${collected ? "取消" : ""}收藏成功");
                      } catch (e, s) {
                        log("$e\n$s");
                        defaultToast(context, "操作失败:$e");
                      } finally {
                        _collecting = false;
                      }
                      setState(() {});
                    },
                    icon: Icon(
                      ((_query.collect ?? 0) > 0)
                          ? Icons.book
                          : Icons.book_outlined,
                      color: theme.textTheme.bodyMedium?.color,
                      shadows: [
                        Shadow(
                          color: Colors.white.withOpacity(1 - _appbarOpacity),
                          offset: const Offset(0, 0),
                          blurRadius: 5,
                        ),
                      ],
                    ),
                  );
                }
                return Container();
              },
            ),
            FutureBuilder(
              future: _fetchFuture,
              builder: (
                BuildContext context,
                AsyncSnapshot<dynamic> snapshot,
              ) {
                if (snapshot.connectionState == ConnectionState.done) {
                  return IconButton(
                    onPressed: () async {
                      Navigator.of(context).push(
                        MaterialPageRoute(
                          builder: (context) => ComicDownloadScreen(
                            comic: _comic.comic,
                            groupChaptersMap: _gcMap,
                          ),
                        ),
                      );
                    },
                    icon: Icon(
                      Icons.download,
                      color: theme.textTheme.bodyMedium?.color,
                      shadows: [
                        Shadow(
                          color: Colors.white.withOpacity(1 - _appbarOpacity),
                          offset: const Offset(0, 0),
                          blurRadius: 5,
                        ),
                      ],
                    ),
                  );
                }
                return Container();
              },
            ),
          ],
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
                child: const Text("从orde头开始"),
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
          comic: _comic.comic,
          chapterUuid: c.uuid,
          initRank: initRank,
          loadChapter: (String comicPathWord, String chapterUuid) async {
            final response = await api.comicChapterData(
              comicPathWord: comicPathWord,
              chapterUuid: chapterUuid,
            );
            return response.chapter;
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

class ComicInfoCard extends StatelessWidget {
  final UIComic comic;

  const ComicInfoCard(this.comic, {super.key});

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
              width: 328 / 3,
              height: 422 / 3,
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
                  comic.name,
                  style: const TextStyle(
                    fontSize: 16,
                  ),
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
                Container(
                  height: 6,
                ),
                Wrap(
                  alignment: WrapAlignment.start,
                  runAlignment: WrapAlignment.start,
                  crossAxisAlignment: WrapCrossAlignment.start,
                  direction: Axis.horizontal,
                  spacing: 10.0,
                  runSpacing: 5.0,
                  children: [
                    _ci(comic.status),
                    _ci(comic.reclass),
                    _ci(comic.region),
                    _ci(comic.restrict),
                    _ci(comic.freeType),
                  ],
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _ci(ClassifyItem ci) {
    return Container(
      padding: const EdgeInsets.only(
        left: 7,
        top: 2,
        right: 7,
        bottom: 2,
      ),
      decoration: BoxDecoration(
        border: Border.all(
          color: Colors.grey.withAlpha(220),
          style: BorderStyle.solid,
          width: .5,
        ),
        borderRadius: const BorderRadius.all(Radius.circular(20)),
      ),
      child: Text(
        ci.display,
        style: TextStyle(
          fontSize: 13,
          color: Colors.grey.withAlpha(220),
        ),
      ),
    );
  }
}
