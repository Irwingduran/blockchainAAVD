// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VotingSystem {
    struct Vote {
        uint256 folio;
        uint256 candidateId;
        address voter;
        bool isCoalition;
    }

    mapping(uint256 => bool) public folioUsed;
    mapping(uint256 => Vote) public votes;
    mapping(address => bool) public hasVoted;
    
    address[4] public validators;
    uint256 public validatorCount;

    bool public votingClosed = false;
    uint256 public votingStartTime;

    event VoteCast(uint256 folio, uint256 candidateId, address voter, bool isCoalition);
    event VotingClosed();

    constructor(address[4] memory _validators) {
        validators = _validators;
        votingStartTime = block.timestamp;
    }

    modifier onlyValidator() {
        require(isValidator(msg.sender), "Not a validator");
        _;
    }

    function isValidator(address _address) public view returns (bool) {
        for (uint i = 0; i < validators.length; i++) {
            if (validators[i] == _address) {
                return true;
            }
        }
        return false;
    }

    function castVote(uint256 _folio, uint256 _candidateId, bool _isCoalition) public {
        require(!votingClosed, "Voting is closed");
        require(!folioUsed[_folio], "Folio already used");
        require(!hasVoted[msg.sender], "Already voted");
        require(_candidateId > 0, "Invalid candidate ID");

        votes[_folio] = Vote(_folio, _candidateId, msg.sender, _isCoalition);
        folioUsed[_folio] = true;
        hasVoted[msg.sender] = true;

        emit VoteCast(_folio, _candidateId, msg.sender, _isCoalition);
    }

    function closeVoting() public onlyValidator {
        validatorCount++;
        if (validatorCount == validators.length) {
            votingClosed = true;
            emit VotingClosed();
        }
    }
}
