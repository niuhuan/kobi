import 'package:flutter/material.dart';
import 'package:flutter_search_bar/flutter_search_bar.dart' as sb;

import '../ffi.io.dart';
import 'components/comic_card.dart';
import 'components/comic_list.dart';
import 'components/comic_pager.dart';

class ComicSearchScreen extends StatefulWidget {
  final String initialQuery;

  const ComicSearchScreen({super.key, required this.initialQuery});

  @override
  _ComicSearchScreenState createState() => _ComicSearchScreenState();
}

class _ComicSearchScreenState extends State<ComicSearchScreen> {
  late var _query = widget.initialQuery;

  late final _searchBar = sb.SearchBar(
    hintText: '搜索',
    inBar: false,
    setState: setState,
    onSubmitted: (value) {
      if (value.isNotEmpty) {
        setState(() {
          _query = value;
        });
      }
    },
    buildDefaultAppBar: _appBar,
  );

  AppBar _appBar(BuildContext context) {
    return AppBar(
      title: Text(_query),
      actions: [
        _searchBar.getSearchAction(context),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      key: Key("search_screen:$_query"),
      appBar: _searchBar.build(context),
      body: ComicPager(fetcher: (offset, limit) async {
        final result = await api.comicSearch(
            qType: "", q: _query, offset: offset, limit: limit);
        return CommonPage<CommonComicInfo>(
          list: result.list
              .map((e) => CommonComicInfo(
                    author: e.author,
                    cover: e.cover,
                    imgType: e.imgType,
                    name: e.name,
                    pathWord: e.pathWord,
                    popular: e.popular,
                    males: e.males,
                    females: e.females,
                  ))
              .toList(),
          total: result.total,
          limit: result.limit,
          offset: result.offset,
        );
      }),
    );
  }
}
