"use client";

import { useEffect, useState } from "react";
import {
  Carousel,
  CarouselApi,
  CarouselContent,
  CarouselItem,
  CarouselPrevious,
  CarouselNext,
} from "@/components/ui/carousel";

interface HeroGalleryProps {
  images: { url: string; alt?: string }[];
}

export function HeroGallery({ images }: HeroGalleryProps) {
  const [api, setApi] = useState<CarouselApi | null>(null);
  const [currentIndex, setCurrentIndex] = useState(0);

  // keep dots in sync
  useEffect(() => {
    if (!api) return;
    const onSelect = () => {
      setCurrentIndex(api.selectedScrollSnap());
    };
    api.on("select", onSelect);
    return () => {
      api.off("select", onSelect);
    };
  }, [api]);

  
  /*
      <div className="absolute bottom-4 left-1/2 -translate-x-1/2 flex space-x-2">
        {images.map((_, i) => (
          <button
            key={i}
            onClick={() => api?.scrollTo(i)}
            className={`w-3 h-3 rounded-full transition-colors ${
              i === currentIndex ? "bg-white" : "bg-white/50 hover:bg-white/70"
            }`}
          />
        ))}
      </div>
      */

    
     /*
      <div className="mt-4 flex justify-center gap-2">
        {images.map((img, i) => (
          <button
            key={i}
            className={`relative w-20 h-12 overflow-hidden rounded-md border-2 ${
              i === currentIndex
                ? "border-white shadow-lg"
                : "border-transparent opacity-70 hover:opacity-100"
            }`}
            onClick={() => api?.scrollTo(i)}
          >
            <img
              src={img.url}
              alt={img.alt ?? ""}
              className="absolute inset-0 w-full h-full object-cover"
            />
          </button>
        ))}
      </div>
      */



    /* onCreate={(api) => setApi(api)} */

  return (
    <Carousel opts={{ loop: false }}

      className="h-[30vh] md:h-[40vh] items-center"
    >
      <CarouselPrevious />
      
      <CarouselContent>
        {images.map((img, idx) => (
          <CarouselItem
            key={idx}
            className="h-[30vh] md:h-[40vh] items-center"
          >
            <img
              src={img.url}
              alt={img.alt ?? ""}
              className="object-cover"
            />
          </CarouselItem>
        ))}
      </CarouselContent>
      
      <CarouselNext />
    </Carousel>
  );
}
