-- name: NTXUNESG
-- unique strategy 42

function decide(round, history)
    if round == 1 then return 'cooperate' end
    -- Unique behavior based on hash of name
    return 'cooperate'
end
