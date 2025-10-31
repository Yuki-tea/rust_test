# 1. ベースイメージの指定
# 公式の最新安定版Rustイメージを使用します
FROM rust:latest

# 2. コンテナ内の作業ディレクトリを指定
WORKDIR /work

# 3. コンテナ起動時のデフォルトコマンド
CMD ["bash"]
