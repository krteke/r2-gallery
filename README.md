# R2 Gallery

一个使用 Cloudflare R2 存储的轻量级图床。

## 部署指南

### 先决条件

- 已安装 [Docker](https://www.docker.com/get-started)。
- 已安装 [Docker Compose](https://docs.docker.com/compose/install/)。

### 第一步：获取项目文件

```bash
# 克隆仓库
git clone <repository-url>
cd r2-gallery

# 从模板创建环境配置文件
cp .env.example .env
```

### 第二步：生成密码哈希

运行以下命令来生成一个安全的密码哈希。

```bash
docker-compose run --rm app cargo run --bin hash_password --features build-tools
```

输入密码后，复制输出的哈希值。稍后将用它来更新 `.env` 文件。

### 第三步：配置 `.env` 文件

打开第一步中创建的 `.env` 文件，并填入所有配置信息。

- `PASSWORD`: 填入第二步中生成的密码哈希。
- `USERNAME`, `JWT_SECRET`, `R2_*`: 填入个人配置。
- `HOST_PORT`: 通过哪个端口来访问应用 (例如 `8080`)。
- 其他变量可以根据需要调整。

### 第四步：启动应用

配置完成后，在项目根目录中运行以下命令来构建并启动应用：

```bash
docker-compose up --build -d
```

- `--build`: 首次启动或更新代码后，需要此参数来构建镜像。
- `-d`: 在后台以分离模式运行容器。

应用现在应该已经成功运行。可以在浏览器中通过在 `.env` 文件中配置的 `HOST_PORT` 来访问它。

例如：`http://localhost:8080`

## 日常管理

- **查看实时日志:**

  ```bash
  docker-compose logs -f
  ```

- **停止并移除容器:**

  ```bash
  docker-compose down
  ```

- **在后台启动服务:**
  ```bash
  docker-compose up -d
  ```
