/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  experimental: {
    typedRoutes: true
  },
  async rewrites() {
    const apiBase =
      process.env.BACKEND_API_BASE_URL || "http://localhost:8080";

    return [
      {
        source: "/api/:path*",
        destination: `${apiBase}/api/:path*`
      }
    ];
  }
};

export default nextConfig;

