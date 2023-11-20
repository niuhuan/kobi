import 'package:flutter/material.dart';
import 'package:kobi/screens/components/comic_pager.dart';

import '../configs/login.dart';
import '../ffi.io.dart';
import 'components/comic_card.dart';

class CollectedComicsAccountScreen extends StatefulWidget {
  const CollectedComicsAccountScreen({Key? key}) : super(key: key);

  @override
  _CollectedComicsAccountScreenState createState() =>
      _CollectedComicsAccountScreenState();
}

class _CollectedComicsAccountScreenState
    extends State<CollectedComicsAccountScreen> {
  @override
  void initState() {
    super.initState();
    loginEvent.subscribe(_setState);
  }

  @override
  void dispose() {
    loginEvent.unsubscribe(_setState);
    super.dispose();
  }

  _setState(_) {
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("收藏列表(账户)"),
      ),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    if (logging) {
      return const Center(child: Text("处理中"));
    }
    if (loginState.state == 0) {
      return const Center(child: Text("未登录"));
    }
    if (loginState.state == 2) {
      return const Center(child: Text("登录失败"));
    }
    return ComicPager(fetcher: (offset, limit) async {
      final result = await api.collectFromAccount(
        freeType: 1,
        ordering: "-datetime_modifier",
        offset: offset,
        limit: limit,
      );
      return CommonPage<CommonComicInfo>(
        list: result.list
            .map((e) => CommonComicInfo(
                  author: e.comic.author,
                  cover: e.comic.cover,
                  imgType: 1,
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
