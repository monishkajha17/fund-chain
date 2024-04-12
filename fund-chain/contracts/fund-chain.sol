// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

contract fund_chain {
    struct Campaign {
        address owner;
        string title;
        string description;
        uint256 target;
        uint256 deadline;
        uint256 amountCollected;
        string image;
        address[] donators;
        uint256[] donations;
    }

    mapping(uint256 => Campaign) public campaigns;

    uint256 public numberofCampaigns = 0;

    // Creates a new campaign with the provided details.
    function createCampaign(
        address _owner,
        string memory _title,
        string memory _description,
        uint256 _target,
        uint256 _deadline,
        string memory _image
    ) public returns (uint256) {
        // Ensure the campaign deadline is in the future.
        require(_deadline > block.timestamp, "The deadline should be a date in the future.");

        // Initialize a new campaign.
        Campaign storage campaign = campaigns[numberofCampaigns];

        // Set campaign details.
        campaign.owner = _owner;
        campaign.title = _title;
        campaign.description = _description;
        campaign.target = _target;
        campaign.deadline = _deadline;
        campaign.amountCollected = 0;
        campaign.image = _image;

        // Increment the number of campaigns.
        numberofCampaigns++;

        // Return the ID of the created campaign.
        return numberofCampaigns - 1;
    }

    // Allows users to donate to a specific campaign.
    function donateToCampaign(uint256 _id) public payable {
        // Ensure a valid amount is sent.
        require(msg.value > 0, "Invalid donation amount.");

        // Retrieve the campaign to donate to.
        Campaign storage campaign = campaigns[_id];

        // Add the donor's address to the list of donators.
        campaign.donators.push(msg.sender);

        // Record the donation amount.
        campaign.donations.push(msg.value);

        // Transfer the donated amount to the campaign owner.
        (bool sent, ) = payable(campaign.owner).call{value: msg.value}("");
        require(sent, "Failed to send donation.");

        // Update the amount collected for the campaign.
        campaign.amountCollected += msg.value;
    }

    // Retrieves the list of donators and their donations for a specific campaign.
    function getDonators(uint256 _id) public view returns (address[] memory, uint256[] memory) {
        return (campaigns[_id].donators, campaigns[_id].donations);
    }

    // Retrieves all campaigns.
    function getCampaigns() public view returns (Campaign[] memory) {
        Campaign[] memory allCampaigns = new Campaign[](numberofCampaigns);

        for (uint256 i = 0; i < numberofCampaigns; i++) {
            allCampaigns[i] = campaigns[i];
        }

        return allCampaigns;
    }
}
