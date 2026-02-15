#!/bin/zsh

# 設定
CONTAINER_NAME="aitalker-postgres"
POSTGRES_USER="api_user_rw"
POSTGRES_PASSWORD="HE4ycm8uCER3"
POSTGRES_PORT=25432
VOLUME_PATH="`pwd`/container_volumes/postgres_data"  # 保存先（自由に変更OK）

# 引数チェック
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 {start|status|stop}"
  exit 1
fi

# 起動前にディレクトリの存在確認
if [[ "$1" == "start" && ! -d "$VOLUME_PATH" ]]; then
  echo "Creating volume directory at $VOLUME_PATH..."
  mkdir -p "$VOLUME_PATH"
fi

case $1 in
  start)
    if podman container exists $CONTAINER_NAME; then
      echo "Starting existing container '$CONTAINER_NAME'..."
      podman start $CONTAINER_NAME
    else
      echo "Creating and starting new PostgreSQL container '$CONTAINER_NAME'..."

      # ボリュームディレクトリ作成
      mkdir -p $VOLUME_PATH

      podman run -d \
        --name $CONTAINER_NAME \
        -e POSTGRES_USER=$POSTGRES_USER \
        -e POSTGRES_PASSWORD=$POSTGRES_PASSWORD \
        -p $POSTGRES_PORT:5432 \
        -v $VOLUME_PATH:/var/lib/postgresql \
        docker.io/library/postgres:18.2-alpine3.22
        # for host is SELinux env.
        # -v $VOLUME_PATH:/var/lib/postgresql:Z \

    fi
    ;;

  status)
    if podman container exists $CONTAINER_NAME; then
      podman inspect -f 'Container "{{.Name}}" is {{.State.Status}}' $CONTAINER_NAME
    else
      echo "Container '$CONTAINER_NAME' does not exist."
    fi
    ;;

  stop)
    if podman container exists $CONTAINER_NAME; then
      echo "Stopping container '$CONTAINER_NAME'..."
      podman stop $CONTAINER_NAME
    else
      echo "Container '$CONTAINER_NAME' does not exist."
    fi
    ;;

  *)
    echo "Invalid argument: $1"
    echo "Usage: $0 {start|status|stop}"
    exit 1
    ;;
esac
