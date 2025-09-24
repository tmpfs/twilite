'use client';
import { usePathname } from "next/navigation";
import NoSsr from '@/components/NoSsr';

export default function WikiRouter() {
  const pathname = usePathname(); 
  const segments = pathname.split("/").filter(Boolean); 
  return (
    <NoSsr>
      { segments.length === 1 ? <WikiIndex /> : <WikiPage /> }
    </NoSsr>
  )
}

function WikiIndex() {
  return (
    <NoSsr>
    <div>
      <h1>Wiki index</h1>
    </div>
    </NoSsr>
  )
}

function WikiPage() {
  const pathname = usePathname(); 
  const segments = pathname.split("/").filter(Boolean); 
 
  return (
    <NoSsr>
    <div>
      <h1>Wiki page</h1>
      <p>{pathname}</p>
      <p>{segments.join("|")}</p>
    </div>
    </NoSsr>
  )
}
