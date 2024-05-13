import 'package:flutter/material.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'package:kobi/screens/components/comic_pager.dart';
import 'package:kobi/screens/recommends_screen.dart';
import 'components/comic_card.dart';

class RankScreen extends StatefulWidget {
  const RankScreen({Key? key}) : super(key: key);

  @override
  _RankScreenState createState() => _RankScreenState();
}

class _RankScreenState extends State<RankScreen> {
  @override
  Widget build(BuildContext context) {
    ThemeData theme = Theme.of(context);
    return DefaultTabController(
      length: 5,
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
                Tab(text: '推荐'),
                Tab(text: '天'),
                Tab(text: '周'),
                Tab(text: '月'),
                Tab(text: '总'),
              ],
            ),
          ),
          const Expanded(
            child: TabBarView(
              children: [
                RecommendsScreen(),
                RankTypeScreen(dateType: "day"),
                RankTypeScreen(dateType: "week"),
                RankTypeScreen(dateType: "month"),
                RankTypeScreen(dateType: "total"),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class RankTypeScreen extends StatelessWidget {
  final String dateType;

  const RankTypeScreen({Key? key, required this.dateType}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ComicPager(fetcher: (offset, limit) async {
      final result =
          await api.rank(dateType: dateType, offset: offset, limit: limit);
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) => CommonComicInfo(
                  author: e.comic.author,
                  cover: e.comic.cover,
                  imgType: e.comic.imgType,
                  name: e.comic.name,
                  pathWord: e.comic.pathWord,
                  popular: e.comic.popular,
                  males: e.comic.males,
                  females: e.comic.females,
                ))
            .toList(),
        total: result.total,
        limit: result.limit,
        offset: result.offset,
      );
    });
  }
}
