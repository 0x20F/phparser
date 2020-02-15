<?php

namespace test\space;


class one
{

}

class two {}

class three {

}

class four {
    function aye() {
        {}
        {}
        {}{}{}{}
        {}{}{}{}
        // Code blocks to try and mess with the lexer
    }
}

class five {{{{{{{{{{{{}}}}}}}}}}}}

use magic/lalala;

class six {
    <html></html>
    <h1>This will never be a thing</h1>
}