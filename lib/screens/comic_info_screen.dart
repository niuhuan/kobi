import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:kobi/screens/components/comic_list.dart';

class ComicInfoScreen extends StatefulWidget {
  final CommonComicInfo comicInfo;

  const ComicInfoScreen({Key? key, required this.comicInfo}) : super(key: key);

  @override
  _ComicInfoScreenState createState() => _ComicInfoScreenState();
}

class _ComicInfoScreenState extends State<ComicInfoScreen> {
  final _scrollController = ScrollController();
  double _scrollOffset = 0;

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
  }

  void _onScroll() {
    setState(() {
      _scrollOffset = _scrollController.offset;
    });
  }

  static const _appHiddenStart = 50.0;
  static const _appHiddenEnd = 150.0;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final _appbarOpacity = _scrollOffset < _appHiddenStart
        ? 1.0
        : _scrollOffset > _appHiddenEnd
            ? 0.0
            : (_appHiddenEnd - _scrollOffset) /
                (_appHiddenEnd - _appHiddenStart);
    return Scaffold(
      body: Stack(children: [
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
                child: Image.network(
                  widget.comicInfo.cover,
                  fit: BoxFit.fill,
                  width: constraints.maxWidth,
                  height: constraints.maxHeight / 3,
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
        ListView(
          controller: _scrollController,
          children: [
            AppBar(
              automaticallyImplyLeading: false,
              elevation: 0,
              backgroundColor: Colors.transparent,
            ),
            CommonComicCard(widget.comicInfo),
          ],
        ),
        Opacity(
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
                actions: const [],
              ),
            ],
          ),
        ),
        Column(
          children: [
            AppBar(
              centerTitle: true,
              leading: IconButton(
                icon: Icon(
                  Icons.arrow_back,
                  color: theme.textTheme.bodyMedium?.color,
                ),
                onPressed: () {
                  Navigator.of(context).pop();
                },
              ),
              backgroundColor: Colors.transparent,
              elevation: .0,
              actions: const [],
            ),
          ],
        ),
      ]),
    );
  }
}


class ComicInfoCard extends StatelessWidget {
  final CommonComicInfo comic;

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
            child: Image.network(
              comic.cover,
              width: 328 / 4,
              height: 422 / 4,
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
