import type { FC } from "react";
import { useState } from 'react'
console.log(useState)

type BookComponent = FC<{ name: string }>

export const Book: BookComponent = ({ name }) => (
  <div>book name{name}</div>
);

{
  console.log(Book)
  {
    const Book = 1;
    console.log(Book)
  }
}

// top level variables

const foo;
let bar;

// global variables
window
document