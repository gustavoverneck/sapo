import Link from "next/link";


export default function Navbar() {
  return (
    <nav className="fixed top-0 left-0 w-full z-50 p-4 bg-black text-white flex justify-between items-center">
      <div className="text-left">
        <Link href="/">Home</Link>
      </div>

      <div className="flex items-center gap-8 text-right">
        <Link href="/contact" 
          className="inline-flex items-center gap-2 inline">
          <span>Contact</span>
        </Link>
      </div>
    </nav>
  );
}