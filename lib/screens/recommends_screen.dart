import 'package:flutter/material.dart';

import '../ffi.io.dart';
import 'components/comic_list.dart';
import 'components/comic_pager.dart';

class RecommendsScreen extends StatelessWidget {
  const RecommendsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final pager = ComicPager(fetcher: (offset, limit) async {
      final result = await api.recommends(offset: offset, limit: limit);
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) => CommonComicInfo(
                  author: e.author,
                  cover: e.cover,
                  imgType: e.imgType,
                  name: e.name,
                  pathWord: e.pathWord,
                  popular: e.popular,
                ))
            .toList(),
        total: result.total,
        limit: result.limit,
        offset: result.offset,
      );
    });
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(0),
        child: Container(),
      ),
      body: pager,
    );
  }
}
