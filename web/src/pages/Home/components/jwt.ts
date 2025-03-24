import { jwtDecode, type JwtPayload } from "jwt-decode";

interface JwtExtendedPayload extends JwtPayload {
  name: string;
}

function parseCookies(): Record<string, string> {
  if (!document.cookie) return {};

  return document.cookie.split("; ").reduce(
    (prev, current) => {
      const [name, ...value] = current.split("=");
      prev[name] = value.join("=");
      return prev;
    },
    {} as Record<string, string>,
  );
}

export function decodeJwt(): JwtExtendedPayload | null {
  const cookies = parseCookies();

  if (Object.keys(cookies).length === 0 || !("access_token" in cookies)) {
    return null;
  }
  return jwtDecode<JwtExtendedPayload>(cookies.access_token);
}
