"use client";

import { useEffect, useState } from "react";
import {
  Carousel,
  CarouselApi,
  CarouselContent,
  CarouselItem,
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


    /* onCreate={(api) => setApi(api)} */

  return (
    <div className="relative w-full">
      {/* Hero Carousel */}
      <Carousel
        className="w-full"
        opts={{ loop: true }}
      >
        <CarouselContent>
          {images.map((img, idx) => (
            <CarouselItem key={idx} className="relative h-[60vh] md:h-[80vh]">
              <img
                src={img.url}
                alt={img.alt ?? ""}
                className="absolute inset-0 w-full h-full object-cover"
              />
              {/* Optional overlay for text */}
              <div className="absolute inset-0 bg-gradient-to-t from-black/40 to-transparent flex items-end p-6">
                <h2 className="text-white text-2xl md:text-4xl font-bold drop-shadow">
                  {img.alt}
                </h2>
              </div>
            </CarouselItem>
          ))}
        </CarouselContent>
      </Carousel>

      {/* Dots */}
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

      {/* Thumbnails */}
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
    </div>
  );
}
