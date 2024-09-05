import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:pull_to_refresh/pull_to_refresh.dart';
import 'comic_card.dart';
import 'comic_list.dart';
import 'commons.dart';

class ComicPager extends StatefulWidget {
  final Future<CommonPage<CommonComicInfo>> Function(BigInt offset, BigInt limit)
      fetcher;

  const ComicPager({Key? key, required this.fetcher}) : super(key: key);

  @override
  State<StatefulWidget> createState() => _ComicPagerState();
}

class _ComicPagerState extends State<ComicPager> {
  final _refreshController = RefreshController(initialRefresh: true);
  final _scrollController = ScrollController();
  final List<CommonComicInfo> _records = [];
  bool finish = false;
  bool error = false;
  int _offset = 0;
  static final BigInt _limit = BigInt.parse("21");

  @override
  void dispose() {
    _refreshController.dispose();
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final lines = comicListLines(context, _records);
    return SmartRefresher(
      controller: _refreshController,
      enablePullDown: true,
      enablePullUp: !finish,
      onRefresh: _onRefresh,
      onLoading: _onLoading,
      header: customerHeader(context),
      footer: customerFooter(context, _records.isNotEmpty),
      child: ListView.builder(
        controller: _scrollController,
        padding: const EdgeInsets.all(0),
        itemCount: lines.length,
        itemBuilder: (context, index) {
          return lines[index];
        },
      ),
    );
  }

  _onRefresh() async {
    try {
      setState(() {
        error = false;
      });
      final resp = await widget.fetcher(
        BigInt.from(0),
        _limit,
      );
      setState(() {
        _records.clear();
        _records.addAll(resp.list);
        _offset = _offset + _limit.toInt();
        finish = resp.total <= _offset;
      });
      _refreshController.refreshCompleted();
      if (finish) {
        _refreshController.loadNoData();
      } else {
        _refreshController.resetNoData();
      }
    } catch (e, s) {
      if (e is PanicException) {
        PanicException e1 = e as PanicException;
        print("$e\n${e1}\n\n$s");
      } else if (e is Exception) {
        Exception e1 = e as Exception;
        print("$e\n${e1}\n\n$s");
      }
      setState(() {
        error = true;
      });
      _refreshController.refreshFailed();
      defaultToast(context, "加载失败");
    }
  }

  _onLoading() async {
    try {
      final resp = await widget.fetcher(
        BigInt.from(_offset),
        _limit,
      );
      setState(() {
        _records.addAll(resp.list);
        _offset = _offset + _limit.toInt();
        finish = resp.total <= _offset;
      });
      _refreshController.loadComplete();
      if (finish) {
        _refreshController.loadNoData();
      } else {
        _refreshController.resetNoData();
      }
    } catch (e, s) {
      print("$e\n$s");
      setState(() {
        error = true;
      });
      _refreshController.loadFailed();
      defaultToast(context, "加载失败");
    }
  }
}

class CommonPage<T> {
  final List<T> list;
  final int total;
  final int limit;
  final int offset;

  const CommonPage({
    required this.list,
    required this.total,
    required this.limit,
    required this.offset,
  });
}

Widget customerHeader(BuildContext context) => CustomHeader(
      builder: (BuildContext context, RefreshStatus? mode) {
        if (mode == RefreshStatus.refreshing) {
          return loadingBanner2("正在接受二次元数据");
        }
        if (mode == RefreshStatus.completed) {
          return loadingBanner2("正在接受二次元数据");
        }
        if (mode == RefreshStatus.failed) {
          return loadingBanner2("未能成功连接二次元");
        }
        if (mode == RefreshStatus.idle) {
          return loadingBanner2("下拉刷新");
        }
        return loadingBanner2("下拉刷新");
      },
    );

Widget customerFooter(BuildContext context, bool recordsIsEmpty) =>
    CustomFooter(
      builder: (BuildContext context, LoadStatus? mode) {
        if (mode == LoadStatus.idle && recordsIsEmpty) {
          return loadingBanner2("");
        }
        if (mode == LoadStatus.canLoading && recordsIsEmpty) {
          return loadingBanner2("");
        }
        if (mode == LoadStatus.loading) {
          return loadingBanner2("");
        }
        return Container();
      },
    );

Widget loadingBanner2(String message) {
  return Column(
    children: [
      Text(
        message,
        style: const TextStyle(
          color: Colors.grey,
          fontSize: 16,
        ),
      ),
    ],
  );
}
