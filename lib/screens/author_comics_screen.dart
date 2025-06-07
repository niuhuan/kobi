import 'package:flutter/material.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/copy_client/dtos.dart';
import '../src/rust/udto.dart';
import 'components/comic_card.dart';
import 'components/comic_pager.dart';

class AuthorComicsScreen extends StatefulWidget {
  final Author author;

  const AuthorComicsScreen(this.author, {Key? key}) : super(key: key);

  @override
  _AuthorComicsScreenState createState() => _AuthorComicsScreenState();
}

class _AuthorComicsScreenState extends State<AuthorComicsScreen> {
  String _ordering = "-datetime_updated"; // 默认按最新更新排序

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("作者: ${widget.author.name}"),
        actions: [
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
        ],
      ),
      body: ComicPager(
        key: Key("author_comics:${widget.author.pathWord}:$_ordering"),
        fetcher: (offset, limit) async {
          final result = await api.exploreByAuthor(
            author: widget.author.pathWord,
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