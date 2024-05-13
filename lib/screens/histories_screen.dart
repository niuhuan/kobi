import 'package:flutter/material.dart';

import '../commons.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'components/comic_card.dart';
import 'components/comic_pager.dart';

class HistoriesScreen extends StatelessWidget {
  const HistoriesScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final pager = ComicPager(fetcher: (offset, limit) async {
      final result = await api.listComicViewLogs(offset: offset, limit: limit);
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) => CommonComicInfo(
          author: stringAuthors(e.comicAuthors),
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
        title: const Text('历史记录'),
      ),
      body: pager,
    );
  }
}
