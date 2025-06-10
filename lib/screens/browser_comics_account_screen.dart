import 'package:flutter/material.dart';
import 'package:kobi/configs/login.dart';
import 'package:kobi/src/rust/api/api.dart';
import 'package:kobi/screens/components/comic_pager.dart';
import 'package:kobi/screens/components/comic_card.dart';

class BrowserComicsAccountScreen extends StatefulWidget {
  const BrowserComicsAccountScreen({Key? key}) : super(key: key);

  @override
  State<BrowserComicsAccountScreen> createState() => _BrowserComicsAccountScreenState();
}

class _BrowserComicsAccountScreenState extends State<BrowserComicsAccountScreen> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("浏览历史"),
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
    return _buildBody1();
  }

  Widget _buildBody1() {
    return Column(
      children: [
        Expanded(
          child: _comicPager(),
        ),
      ],
    );
  }

  Widget _comicPager() {
    return ComicPager(
      key: const Key("browser_comics_account"),
      pageSize: 18,
      fetcher: (offset, limit) async {
        final result = await browser(
          offset: offset,
          limit: limit,
        );
        return CommonPage<CommonComicInfo>(
          list: result.list.map((e) => CommonComicInfo(
            author: e.comic.author, // Browse 模型中没有作者信息
            cover: e.comic.cover, // 使用 pathWord 作为封面占位符
            imgType: 1,
            name: e.comic.name,
            pathWord: e.comic.pathWord,
            popular: e.comic.popular, // Browse 模型中没有热度信息
            males: e.comic.males, // Browse 模型中没有性别信息
            females: e.comic.females, // Browse 模型中没有性别信息
          )).toList(),
          total: result.total.toInt(),
          limit: result.limit.toInt(),
          offset: result.offset.toInt(),
        );
      },
    );
  }
} 