import 'dart:convert';

import 'package:flutter/material.dart';

import '../bridge_generated.dart';
import '../ffi.io.dart';
import 'components/comic_list.dart';
import 'components/comic_pager.dart';

class BooksScreen extends StatelessWidget {
  const BooksScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final pager = ComicPager(fetcher: (offset, limit) async {
      final result = await api.listComicViewLogs(offset: offset, limit: limit);
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) =>
            CommonComicInfo(
              author: _mapAuthor(List.of(jsonDecode(e.comicAuthors)).cast()),
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
    return Scaffold(
      appBar: AppBar(
        title: const Text("历史记录"),
      ),
      body: pager,
    );
  }
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
