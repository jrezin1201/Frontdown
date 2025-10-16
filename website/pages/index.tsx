import Link from 'next/link';

export default function Home() {
  return (
    <main>
      <h1>RavensOne</h1>
      <p>One language. One stack. Ship faster.</p>
      <nav>
        <Link href="/docs">Docs</Link>
      </nav>
    </main>
  );
}
