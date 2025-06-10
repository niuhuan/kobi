import 'package:flutter/material.dart';
import 'components/flutter_search_bar_base.dart' as sb;
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'components/comic_card.dart';
import 'components/comic_pager.dart';

class AuthorSearchScreen extends StatefulWidget {
  const AuthorSearchScreen({Key? key}) : super(key: key);

  @override
  _AuthorSearchScreenState createState() => _AuthorSearchScreenState();
}

class _AuthorSearchScreenState extends State<AuthorSearchScreen> {
  String _query = "";
  String _ordering = "-datetime_updated"; // 默认按最新更新排序

  late final _searchBar = sb.SearchBar(
    hintText: '按作者名称搜索',
    inBar: false,
    setState: setState,
    onSubmitted: (value) {
      setState(() {
        _query = value;
      });
    },
    buildDefaultAppBar: _appBar,
  );

  AppBar _appBar(BuildContext context) {
    return AppBar(
      title: Text(_query.isEmpty ? "按作者名称搜索" : _query),
      actions: [
        if (_query.isNotEmpty)
          PopupMenuButton<String>(
            onSelected: (String value) {
              setState(() {
                _ordering = value;
              });
            },
            itemBuilder: (BuildContext context) => [
              const PopupMenuItem(
                value: "-datetime_updated",
                child: Text("最新更新"),
              ),
              const PopupMenuItem(
                value: "datetime_updated",
                child: Text("最早更新"),
              ),
              const PopupMenuItem(
                value: "-popular",
                child: Text("最热门"),
              ),
              const PopupMenuItem(
                value: "popular",
                child: Text("最冷门"),
              ),
            ],
          ),
        _searchBar.getSearchAction(context),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _searchBar.build(context),
      body: _query.isEmpty
          ? const Center(
              child: Text(
                "请输入作者名称搜索",
                style: TextStyle(
                  fontSize: 16,
                  color: Colors.grey,
                ),
              ),
            )
          : ComicPager(
              key: Key("author_search:$_query:$_ordering"),
              fetcher: (offset, limit) async {
                final result = await api.exploreByAuthorName(
                  authorName: _query,
                  ordering: _ordering,
                  offset: offset,
                  limit: limit,
                );
                return CommonPage<CommonComicInfo>(
                  list: result.list
                      .map((e) => CommonComicInfo(
                            author: e.author,
                            cover: e.cover,
                            imgType: 1,
                            name: e.name,
                            pathWord: e.pathWord,
                            popular: e.popular.toInt(),
                            males: e.males,
                            females: e.females,
                          ))
                      .toList(),
                  total: result.total,
                  limit: result.limit,
                  offset: result.offset,
                );
              },
            ),
    );
  }
} 