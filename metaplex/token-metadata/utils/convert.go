package main

import (
	"bytes"
	"compress/zlib"
	"fmt"
	"io"
	"log"
	"os"
)

func main() {
	// Read all data from stdin
	bs, err := io.ReadAll(os.Stdin)
	if err != nil {
		log.Fatal(err)
	}

	// Compress the data using zlib
	var b bytes.Buffer
	w := zlib.NewWriter(&b)
	_, err = w.Write(bs)
	if err != nil {
		log.Fatal(err)
	}
	err = w.Close()
	if err != nil {
		log.Fatal(err)
	}

	// Get the compressed data
	compressedData := b.Bytes()

	// Build a string representation of the compressed byte slice
	fmt.Print("const bytes = new Uint8Array([")
	for i, c := range compressedData {
		if i > 0 {
			fmt.Print(",")
		}
		fmt.Print(c)
	}
	fmt.Println("]);")
}
