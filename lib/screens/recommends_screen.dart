import 'package:flutter/material.dart';

import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'components/comic_card.dart';
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
                  females: e.females,
                  males: e.males,
                ))
            .toList(),
        total: result.total,
        limit: result.limit,
        offset: result.offset,
      );
    });
    return pager;
  }
}
