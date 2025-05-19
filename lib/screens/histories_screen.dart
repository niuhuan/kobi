import 'package:flutter/material.dart';

import '../commons.dart';
import '../src/rust/api/api.dart' as api;
import '../src/rust/udto.dart';
import 'components/comic_card.dart';
import 'components/comic_pager.dart';

class HistoriesScreen extends StatefulWidget {
  const HistoriesScreen({super.key});

  @override
  State<HistoriesScreen> createState() => _HistoriesScreenState();
}

class _HistoriesScreenState extends State<HistoriesScreen> {
  final _pagerController = ComicPagerController();

  Future<void> _showDeleteConfirmDialog(
      BuildContext context, String title, String content, VoidCallback onConfirm) async {
    final result = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(title),
        content: Text(content),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(false),
            child: const Text('取消'),
          ),
          TextButton(
            onPressed: () => Navigator.of(context).pop(true),
            child: const Text('删除'),
          ),
        ],
      ),
    );
    if (result == true) {
      onConfirm();
    }
  }

  @override
  Widget build(BuildContext context) {
    final pager = ComicPager(
      controller: _pagerController,
      fetcher: (offset, limit) async {
        final result = await api.listComicViewLogs(
            offset: offset.toInt(), limit: limit.toInt());
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
      },
      onLongPress: (comic, index) async {
        await _showDeleteConfirmDialog(
          context,
          '删除历史记录',
          '确定要删除《${comic.name}》的历史记录吗？',
          () async {
            await api.deleteComicViewLog(pathWord: comic.pathWord);
            if (context.mounted) {
              ScaffoldMessenger.of(context).showSnackBar(
                const SnackBar(content: Text('删除成功')),
              );
              // 从当前页开始刷新
              await _pagerController.refreshFromLimit(index);
            }
          },
        );
      },
    );

    return Scaffold(
      appBar: AppBar(
        title: const Text('历史记录'),
        actions: [
          IconButton(
            icon: const Icon(Icons.delete_sweep),
            onPressed: () async {
              await _showDeleteConfirmDialog(
                context,
                '清空历史记录',
                '确定要清空所有历史记录吗？此操作不可恢复。',
                () async {
                  await api.deleteAllComicViewLogs();
                  if (context.mounted) {
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('清空成功')),
                    );
                    // 清空所有记录并刷新
                    _pagerController.filterComics((_) => false);
                  }
                },
              );
            },
          ),
        ],
      ),
      body: pager,
    );
  }
}
