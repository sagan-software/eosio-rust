(function() {var implementors = {};
implementors["eosio"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/iterator/trait.Iterator.html\" title=\"trait core::iter::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"eosio/struct.PrimaryTableIterator.html\" title=\"struct eosio::PrimaryTableIterator\">PrimaryTableIterator</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"eosio/trait.TableRow.html\" title=\"trait eosio::TableRow\">TableRow</a>,&nbsp;</span>",synthetic:false,types:["eosio::table_primary::PrimaryTableIterator"]},{text:"impl&lt;'a, K, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/iterator/trait.Iterator.html\" title=\"trait core::iter::iterator::Iterator\">Iterator</a> for <a class=\"struct\" href=\"eosio/struct.SecondaryTableIterator.html\" title=\"struct eosio::SecondaryTableIterator\">SecondaryTableIterator</a>&lt;'a, K, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: <a class=\"trait\" href=\"eosio/trait.SecondaryTableKey.html\" title=\"trait eosio::SecondaryTableKey\">SecondaryTableKey</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"eosio/trait.TableRow.html\" title=\"trait eosio::TableRow\">TableRow</a>,&nbsp;</span>",synthetic:false,types:["eosio::table_secondary::SecondaryTableIterator"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
